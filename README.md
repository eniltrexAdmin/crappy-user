# Crappy User microservice

Micro for user authentication.

This is an independent micro from the crappy, that will be done directly
with ES + CQRS, following a little bit the schema from "Practical microservices".

This will be an app that will register used registered events and save them
in the message store, then some aggregator, that I haven't decided yet where it will
be, probably in this very micro, will create the read model.

The aggregator on the book are done via polling, I might avoid that by just
doing a synchronous call, but it should be independent enough that if I change 
my mind, I can change it easily.

# Big picture

This whole micro might be replaced fully by a GOlang micro, just to show off.
deployed in production by a kubernetes ecosystem in aws.

## Decision that should be easy to change in the future:

- The message store DB
- The aggregator

## CQRS and ES
I am using this package to implement CQRS and ES: https://doc.rust-cqrs.org/
What I am mainly interested in is in saving the messages in the message store, I don't
want to do that by myself, but on the other hand, I don't want to overly attack myself
to this package in case there are limitations.

Advantages:
test framework out of the box
applies the events without having to write yourself this logic
(or the test would not have pass!)

Disadvantages:
missing application layer. I might need to query the view data for example
to see that the user is not already registered.
(like a "get" also, the same function) and this can't be done in the aggregate
(this might force me to not to use this package anymore).

Errors might become also domain events to be saved, but well, I can still
do it this way while keeping using this package



## Commands:

sqlx migrate --source src/infrastructure/persistence/postgres/migrations add create_users_view
