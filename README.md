# Crappy User microservice

Micro for user authentication.

This is an independent micro from the crappy ecosystem. 


## Decision that should be easy to change in the future:

- The message store DB

## CQRS and ES

### About  https://doc.rust-cqrs.org/

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
to see that the user is not already registered (see below)

The main reason I am not using though is so I can have control of it, + I can learn in the process.
Using a "framework" on your domain is probably a bad idea. As soon as it doesn't feed your needs, you are
fucked up.

For example, now I can put the command handlers in the application layer. I can make the command handlers
to never return errors but instead always register the errors as domain events "xxx failed" that can
have their own aggregator.


### Translating Garofolo architecture to hex architecture:

Those are the pieces on Garofolo Architecture:
- App
- Components
- Message Store
- Aggregator
- View Data

Now. Message Store and View Data are just DB. So we end up with basically 3 big pieces:
- App
- Components
- Aggregators

App creates the commands, that components subscribe to and create the domain events, that
aggregators subscribe to and create the view data (that the app reads on queries)

#### My way

I have 3 class citizens: 
- command handlers, (app + component in the book) 
- queries, (app + view data in the book)
- aggregators. (aggregator + view data in the book)

Each one of them is made of the 3 pieces of hex architecture:
- infrastructure
- application
- domain.

I am not saving commands in the message store, so some command handlers, might do some stuff before actually
calling the domain (like see below the avoiding duplicated).

How does it look like:

- Command (+writers+) Handlers

It starts with infra on the controllers to generate the command (stretch here, but just fast) and call
the command handler in the application. (that's the "app" part). Then the application will call the domain
to do its shit, and register the domain events in the message store back in the infrastructure. (component part).
The domain doing is shsit needs to take into account what happens when the same event comes twice. Idempotence.

- Queries

It also start with the infra, the controllers, calling the query handler, that lives in the application. The
application loads the view data and displays it, (modelled in the domain too).

- Aggregators

(TBC) It starts with the infrastructure, and asynchronous process that polls constantly the messages
on the message store (from the last processed message on that stream) and creates the view data
while processing the messages (it doesn't matter though if it processes the same message twice, the final 
state must be the same, idempotence)

##### Not going this way:

If I had it separated, I would have then 4 class citizens with the component, and each would have the 3 pieces.
Actually each of the piece could live in separate repos. How would it look like. Well the same as above
but with the difference:

- Command ~~Handlers~~ writers (app): they would just write the command in the message store not executing the domain at all. (they
will have contact with the domain since I've decided commands are first class domain citizens)

- Components (the actual command handlers): will look pretty much like the aggregators: they would start on the infra 
subscribed to the command stream and execute then the domain as command handler

Each of those things could live in different machines, only to access that small piece of infra that they need
the message store. I might go this way for my next project.


### About avoiding registering an email that has been already registered:

From Garofolo book:
```html
(p. 99)
We still haven\'t justified the decision to use the View Data to validate
that the emails supplied during registration aren't duplicated, a direct
contradiction of what we said "Here" when first discussing the pieces of our architecture.
```
Well , he comes to say that we used "application" + "view data" on his graph, which doesn't
violate the unidirectional flow or rules of what components are in his graph.

It's the trade off:
```html
How likely is the thing youre trying to prevent?
How bad is it if the thing youre trying to prevent happens?
whose constraint is it anyway?
what does correct mean?
```

So what am I going to do? Check teh view data in the application layer when executing the command handler.
So this won't reach the domain in that case, no even domain will be generated.

how would I proceed if I were not read view data in the application layer for avoiding email duplicates?
(take into account, that view data is eventually consistent, so duplicates can still occur while using view data)
well form the book:
```html
if unique email addresses are truly a fundamental propery of identities [he meant it 
belong to the domain, not an application concern.. which well, to me kind of its oibvious this belongs to the domain
] , then it's up to the Component that owns identities to enforce that property.

A component could go to the message store and look at every identity in the system and see if their current
email address matches the one in this new registration command and it would reject the command with a
registration rejected event. That would be costly to do on the fly, so at startup time it might compile
a list of used email addressess and stash list in its own database table [isn't that already the view
table though?] no one else would be allowed to use that stach of course, because that would violate
the component's autonomy. But it can do whatever it wants internally
```
Well, since I am not separating the components in different processes/ machines, that's too much, but that's
what I would do (without the pre-loading list -or with the pre-loading list on a cache layer!!! that's a good idea!).
Oh, also my view data.. is not that "eventually consistent", it's pretty much sync - well I still don't know how to 
start doing the aggregators.


## Commands

sqlx migrate --source src/infrastructure/persistence/postgres/migrations add create_users_view
