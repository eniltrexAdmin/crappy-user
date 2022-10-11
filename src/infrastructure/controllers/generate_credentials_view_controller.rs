use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use crate::actix::controllers::NoContentResponse;
use crate::application::user_credentials_aggregator;
use crate::domain::{EventStoreInterface, UserViewRepositoryError};
use crate::event_store_postgres::EventStorePostgres;
use crate::user_view_repository_postgres::UserViewPostgresRepository;

// Temporal way to generate view. I should create subscriber etc.

#[tracing::instrument(name = "Generate View Controller.", skip(pool))]
pub async fn generate_credentials_view(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UserViewRepositoryError> {
    let postgre_event_store_repository = EventStorePostgres::new_event_store(&pool);
    let view_postgre_repository = UserViewPostgresRepository{pool: &pool};

    let events = postgre_event_store_repository.load_all_events(0).await?;
    for event in events {
        user_credentials_aggregator(
            event,
            &view_postgre_repository,
        ).await?;
    }

    Ok(HttpResponse::Created().json(NoContentResponse {}))
}
