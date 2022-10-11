use crate::domain::{EventStoreInterface, LogInCommand, User, UserEmail, UserEventStoreRepository, UserId, UserViewRepositoryError, UserViewRepositoryInterface};

#[tracing::instrument(
name = "Authenticate application",
skip(password, view_repository, user_event_store_repository)
)]
pub async fn authenticate_user(
    email: String,
    password_attempt: String,
    view_repository: &impl UserViewRepositoryInterface,
    user_event_store_repository: &UserEventStoreRepository<impl EventStoreInterface<User>>
) -> Result<(), UserViewRepositoryError>  {
   //I thing I shoudl have in my head that this whole file could become an standalone
    // service with it's own infra, it just needs to connect to the appropriate DBs.

    // CREDENTIALS NEED TO HAVE UUID or I CANT SAVE THE EVENTS WHERE THEY BELONG.

    // will do:
    // 1. load user credentials   // Alt: load user without UUID how??? I can't!!!!
    // 2. ensure credentials found // Alt: ensure user found
    // if not write Event C on which stream??  // if not write event on which stream????
    // 3. validate password/ // Alt: validate password
    // if not, write event B // if not write event B
    // 4. REturn JWT, write event A // return JWT.

    let user_id = UserId::new(command.id);
    let user_email = UserEmail::new(&email)?;
    let credentials_view = view_repository.retrieve_user_credential_by_email(&user_email)?;


    let password_hash = PasswordHash::new(&user_password.hash_string).unwrap();

    let alg: &[&dyn PasswordVerifier] = &[&Argon2::default()];



    user_event_store_repository
        .save_events(user_id, events)
        .await?;
    Ok(())
}
