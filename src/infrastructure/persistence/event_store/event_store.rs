// use cqrs_es::mem_store::MemStore;
// use cqrs_es::{EventEnvelope, Query};
// use crate::domain::User;
//
// // let event_store = MemStore::<BankAccount>::default();
//
// struct SimpleLoggingQuery {}
//
// #[async_trait]
// impl Query<User> for SimpleLoggingQuery {
//     async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<User>]) {
//         for event in events {
//             println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
//         }
//     }
// }