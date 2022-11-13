use crate::actix::controllers::{SuccessfulAuthenticationResponse};
use crate::application::{AuthenticateUserApplicationRequest, crappy_authenticate_user};
use crate::domain::{UserDomainError, UserEventStoreRepository};
use crate::event_store_postgres::EventStorePostgres;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::user_view_repository_postgres::UserViewPostgresRepository;

#[tracing::instrument(name = "Post to Authenticate User.", skip(pool))]
pub async fn authenticate_user(
    request: web::Json<AuthenticateUserApplicationRequest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UserDomainError> {
    let postgres_event_store_repository = EventStorePostgres::new_event_store(&pool);
    let user_repository = UserEventStoreRepository {
        store: postgres_event_store_repository,
    };

    let postgres_view_repository = UserViewPostgresRepository{
        pool: &pool
    };

    let result = crappy_authenticate_user(
        request.into_inner(),
        &postgres_view_repository,
        &user_repository
    ).await?;
    Ok(HttpResponse::Accepted().json(SuccessfulAuthenticationResponse {token: result}))
}


