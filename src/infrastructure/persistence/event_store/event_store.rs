// use crate::domain::{EventEnvelope, EventStoreInterface, User};
// use uuid::Uuid;
//
// pub struct EventStorePostgres {}
//
// impl EventStoreInterface<User> for EventStorePostgres {
//     async fn load_events(
//         &self,
//         aggregate_id: &Uuid,
//     ) -> Result<Vec<EventEnvelope<A>>, crate::domain::event_sourced_aggregate::Error> {
//         todo!()
//     }
//
//     async fn save_events(
//         &self,
//         events: Vec<EventEnvelope<A>>,
//     ) -> Result<(), crate::domain::event_sourced_aggregate::Error> {
//     }
// }
