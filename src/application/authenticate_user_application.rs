use crate::domain::{EventStoreInterface, User, UserEventStoreRepository, UserViewRepositoryError, UserViewRepositoryInterface};

#[tracing::instrument(
name = "Authenticate application",
skip(password_attempt, view_repository, user_event_store_repository)
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



    // I tihnk I know what to do!, I will make a sync call to a command handler
    // that will execute the stuff! maybe it's exra overhead, and think on how to
    // maybe avoid this file "xxx_application" all together, but  I think I like it
    // lets seee where we go

    // user_event_store_repository
    //     .save_events(user_id, events)
    //     .await?;
    Ok(())
}
