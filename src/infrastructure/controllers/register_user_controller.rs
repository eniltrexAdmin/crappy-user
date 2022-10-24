use crate::actix::controllers::NoContentResponse;
use crate::application::register_user_command_handler;
use crate::domain::{RegisterUserCommand, UserDomainError, UserEventStoreRepository};
use crate::event_store_postgres::EventStorePostgres;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[tracing::instrument(name = "Post to Register User.", skip(request, pool))]
pub async fn register_user(
    request: web::Json<RegisterUserCommand>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UserDomainError> {
    let postgre_event_store_repository = EventStorePostgres::new_event_store(&pool);
    let user_repository = UserEventStoreRepository {
        store: postgre_event_store_repository,
    };

    register_user_command_handler(&user_repository, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(NoContentResponse {}))
}
