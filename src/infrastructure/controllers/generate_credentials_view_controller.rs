use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use crate::actix::controllers::NoContentResponse;
use crate::application::user_credentials_aggregator;
use crate::domain::{UserViewRepositoryError};
use crate::event_store_postgres::EventStorePostgres;
use crate::user_view_repository_postgres::UserViewPostgresRepository;

// Temporal way to generate view. I should create subscriber etc.

#[tracing::instrument(name = "Generate View Controller.", skip(pool))]
pub async fn generate_credentials_view(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UserViewRepositoryError> {
    let postgre_event_store_repository = EventStorePostgres::new_event_store(&pool);
    let view_postgre_repository = UserViewPostgresRepository{pool: &pool};
    user_credentials_aggregator(
        postgre_event_store_repository,
        view_postgre_repository,
        0
    ).await?;
    Ok(HttpResponse::Created().json(NoContentResponse {}))
}
