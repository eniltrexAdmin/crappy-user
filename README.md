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

