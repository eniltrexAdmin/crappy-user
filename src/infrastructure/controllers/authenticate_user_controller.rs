use crate::actix::controllers::{NoContentResponse, SuccessfulAuthenticationResponse};
use crate::application::authenticate_user_command_handler;
use crate::domain::{AuthenticateUserCommand, UserDomainError, UserEventStoreRepository};
use crate::event_store_postgres::EventStorePostgres;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[tracing::instrument(name = "Post to Authenticate User.", skip(request, pool))]
pub async fn authenticate_user(
    request: web::Json<AuthenticateUserCommand>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UserDomainError> {
    let postgre_event_store_repository = EventStorePostgres::new_event_store(&pool);
    let user_repository = UserEventStoreRepository {
        store: postgre_event_store_repository,
    };

    let result = authenticate_user_command_handler(&user_repository, request.into_inner()).await?;
    Ok(HttpResponse::Accepted().json(SuccessfulAuthenticationResponse {token: result}))
}
