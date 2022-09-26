use crate::domain::{EventEnvelope, User, UserCredentialsView, UserViewRepositoryError, UserViewRepositoryInterface};
use crate::domain::UserEvent::RegisteredUser;

#[tracing::instrument(
name = "User Credentials aggregator",
skip(view_repository)
)]
pub async fn user_credentials_aggregator(
    event: EventEnvelope<User>,
    view_repository: &impl UserViewRepositoryInterface,
) -> Result<(), UserViewRepositoryError>  {
    match event.payload {
        RegisteredUser(user_registered_domain_event) => {
            let user_credentials_view: UserCredentialsView = user_registered_domain_event.into();
            view_repository.save_view(user_credentials_view).await?;
        }
    }
    Ok(())
}
