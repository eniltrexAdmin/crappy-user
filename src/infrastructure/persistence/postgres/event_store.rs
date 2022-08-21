use std::fmt::Debug;
use crate::domain::{DomainEvent, EventEnvelope, EventSourcedAggregate, EventStoreError, EventStoreInterface};
use async_trait::async_trait;
use std::marker::PhantomData;
use sqlx::PgPool;
use uuid::Uuid;

pub struct EventStorePostgres<'a, A>
where
    A: EventSourcedAggregate + Send + Sync,
{
    pub pool: &'a PgPool,
    _phantom: PhantomData<A>,
}

#[async_trait]
impl<A> EventStoreInterface<A> for EventStorePostgres<'_, A>
where
    A: EventSourcedAggregate + Debug,
{
    async fn load_events(
        &self,
        aggregate_id: &Uuid,
    ) -> Result<Vec<EventEnvelope<A>>, EventStoreError> {
        todo!()
    }

    #[tracing::instrument(
    name = "Saving Events to PostgresSQL",
    skip(self)
    )]
    async fn save_events(&self, events: Vec<EventEnvelope<A>>) -> Result<(), EventStoreError> {
        // I should start "open transaction"
        for event_envelope in events {
            // stupidity, I dont know why I need to convert this types, should be the same!!!!
            let aggregate_id = sqlx::types::Uuid::parse_str(&event_envelope.aggregate_id.to_string())
                .map_err(|error| {
                EventStoreError::SerializationError(
                    error.to_string()
                )
            })?;
            let payload =  serde_json::to_value(&event_envelope.payload)
                .map_err(|serde_error| {
                    EventStoreError::SerializationError(
                        serde_error.to_string()
                    )
                })?;
            let metadata = serde_json::to_value(&event_envelope.metadata)
                .map_err(|serde_error| {
                    EventStoreError::SerializationError(
                        serde_error.to_string()
                    )
                })?;
            let event_type = event_envelope.payload.event_type();
            sqlx::query!(
                r#"
                INSERT INTO events(
                    aggregate_type,
                    aggregate_id,
                    event_type,
                    event_version,
                    payload,
                    metadata,
                    timestamp
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                A::aggregate_type(),
                aggregate_id,
                event_type,
                event_envelope.payload.event_version(),
                payload,
                metadata,
                event_envelope.occurred_at
            ).execute(self.pool)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to execute query: {:?}", e);
                    EventStoreError::DatabaseConnectionError(
                        e.as_database_error().unwrap().to_string()
                    )
                })?;
        }
        Ok(())
    }
}
