// use postgres_es::PostgresCqrs;
// use crate::domain::{EventStoreInterface, RegisterUserCommand, User, UserDomainError, UserEventStoreRepository};

// #[tracing::instrument(
// name = "Register User Command Handler",
// skip(match_request_repository)
// )]
// pub async fn register_user_command_handler(
//     event_store_repository: UserEventStoreRepository<EventStoreInterface<User>>,
//     command: &RegisterUserCommand,
//     // cqrs: PostgresCqrs<User>
// ) -> Result<(), UserDomainError> {
//     let register_user_command = command.try_into()?;
//     // cqrs.execute_with_metadata();
//     // User::handle(register_user_command, &());
//     // return match_request_repository.save(match_request).await;
// }





// async fn process_command(
//     cqrs: PostgresCqrs<BankAccount>,
//     command: BankAccountCommand,
// ) -> Result<(), AggregateError<BankAccountError>> {
//     let mut metadata = HashMap::new();
//     metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
//
//     cqrs.execute_with_metadata("agg-id-F39A0C", command, metadata).await
// }
//
