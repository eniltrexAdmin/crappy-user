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

So far, I am using this even in my domain, which I dont like so:
TODO: try to move th epackage to the infra part and in my domain
have all the domain events to implement my own domain trait, then try to move
out of the domain teh cqrs_es package... because I am not respecting hex architecture now.

