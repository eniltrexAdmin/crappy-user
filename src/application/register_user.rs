use crate::domain::UserRegistrationError;



pub async fn register_user_command_handler() -> Result<(), UserRegistrationError> {
    Ok(())
}



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
