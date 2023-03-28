use secrecy::ExposeSecret;
use crate::application::AuthenticateUserCommand;
use crate::domain::{UserDomainError, UserId, UserDomainEvent, UserRepository};

#[tracing::instrument(
name = "Authenticate User Command Handler",
skip(user_event_store_repository)
)]
pub async fn authenticate_user_command_handler(
    user_event_store_repository: &impl UserRepository,
    command: AuthenticateUserCommand,
) -> Result<UserDomainEvent, UserDomainError> {
    let user_id = UserId::new(command.id);
    let user = user_event_store_repository.load(user_id).await?;

    let events = user.authenticate_user(&command.password_attempt.expose_secret())?;
    let authentication_result = events.first().unwrap().clone();
    user_event_store_repository
        .save_events(user_id, events)
        .await?;
    return Ok(authentication_result);
}

// I find this test stupid. It helped to realize I needed a trait for the User Repository
// and to realize that I'd like to generalize this trait as a 'aggregate sourced repository'
// more todos.. but this is just to show how mocks work. This tests is almost integration, is all
// infra, not unit test.
// #[cfg(test)]
// mod tests {
//     use claim::{assert_ok};
//     use secrecy::SecretString;
//     use crate::application::authenticate_user_command_handler;
//     use crate::domain::{AuthenticateUserCommand, UserId};
//     use crate::domain::MockUserRepository;
//     use crate::domain::user_tests::tests;
//     use crate::domain::user_tests::tests::simulate_fetch_user;
//
//     #[tokio::test]
//     async fn test_command_handler() {
//         let user = tests::default_user();
//         let closure_return_from_repo = |user_id:UserId| {
//             let user = simulate_fetch_user(
//                 user_id.value().to_owned(),
//                 "francesc.travesa@mymail.com",
//                 "password_hash"
//             );
//             Ok(user)
//         };
//         let command = AuthenticateUserCommand::new(
//             user.id().value().to_owned(),
//             "user@test.com".to_string(),
//             SecretString::from("password".to_string())
//         );
//
//         let mut mock_repository = MockUserRepository::new();
//         mock_repository.expect_load().returning(closure_return_from_repo);
//         mock_repository.expect_save_events().returning(|_,_| {Ok(())});
//
//         let result =
//             authenticate_user_command_handler(&mock_repository, command)
//                 .await;
//         // assert_ok_eq!(result, UserDomainEvent::UserAuthenticationFailed(UserAuthenticationFailed{
//         //     id: user.id().value().to_owned(),
//         //     occurred_at:  Utc::now()
//         // }));
//         assert_ok!(result);
//     }
// }
