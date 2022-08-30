use crate::domain::{EventEnvelope, EventSourcedAggregate, EventStoreError, EventStoreInterface};
use crate::serialized_event::{serialize_events, SerializedEvent};
use async_trait::async_trait;
use futures::TryStreamExt;
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use std::fmt::Debug;
use std::marker::PhantomData;
use uuid::Uuid;

pub struct EventStorePostgres<'a, A>
where
    A: EventSourcedAggregate + Send + Sync,
{
    pub pool: &'a PgPool,
    _phantom: PhantomData<A>,
}
impl<'a, A> EventStorePostgres<'a, A>
where
    A: EventSourcedAggregate + Send + Sync,
{
    pub fn new_event_store(pool: &'a PgPool) -> Self {
        EventStorePostgres {
            pool,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<A> EventStoreInterface<A> for EventStorePostgres<'_, A>
where
    A: EventSourcedAggregate + Debug,
{
    #[tracing::instrument(name = "Loading events from the message store", skip(self))]
    async fn load_events(
        &self,
        aggregate_id: &Uuid,
    ) -> Result<Vec<EventEnvelope<A>>, EventStoreError> {
        let mut rows = sqlx::query("SELECT aggregate_type, aggregate_id, event_type, event_version, payload, metadata, timestamp
                  FROM events
                  WHERE aggregate_type = $1 AND aggregate_id = $2
                  ORDER BY sequence")
            .bind(A::aggregate_type())
            .bind(aggregate_id)
            .fetch(self.pool);
        let mut result: Vec<EventEnvelope<A>> = Default::default();
        while let Some(row) = rows.try_next().await? {
            result.push(SerializedEvent::from_row(&row)?.try_into()?);
        }

        Ok(result)
    }

    #[tracing::instrument(name = "Loading all the events for the aggregate", skip(self))]
    async fn load_all_events(
        &self,
        last_event_read: u64
    ) -> Result<Vec<EventEnvelope<A>>, EventStoreError> {
        let mut rows = sqlx::query("SELECT aggregate_type, aggregate_id, event_type, event_version, payload, metadata, timestamp
                  FROM events
                  WHERE aggregate_type = $1 AND sequence > $2
                  ORDER BY sequence")
            .bind(A::aggregate_type())
            .bind(last_event_read)
            .fetch(self.pool);
        let mut result: Vec<EventEnvelope<A>> = Default::default();
        while let Some(row) = rows.try_next().await? {
            result.push(SerializedEvent::from_row(&row)?.try_into()?);
        }

        Ok(result)
    }

    #[tracing::instrument(name = "Saving Events to PostgresSQL", skip(self))]
    async fn save_events(&self, events: Vec<EventEnvelope<A>>) -> Result<(), EventStoreError> {
        let serialized_events = serialize_events(&events)?;
        let mut tx: Transaction<Postgres> = sqlx::Acquire::begin(self.pool).await?;
        for serialized_event in serialized_events {
            sqlx::query("INSERT INTO events (aggregate_type, aggregate_id, event_type, event_version, payload, metadata, timestamp)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)")
                .bind(serialized_event.aggregate_type)
                .bind(serialized_event.aggregate_id)
                .bind(serialized_event.event_type)
                .bind(serialized_event.event_version)
                .bind(serialized_event.payload)
                .bind(serialized_event.metadata)
                .bind(serialized_event.occurred_at)
                .execute(&mut tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}

impl From<sqlx::Error> for EventStoreError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!(
            "SQLX error: {:?}",
            err.as_database_error().unwrap().to_string()
        );
        EventStoreError::DatabaseConnectionError(err.as_database_error().unwrap().to_string())
    }
}
