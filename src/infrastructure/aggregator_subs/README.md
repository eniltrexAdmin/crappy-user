# The aggregator

===
The idea of the aggregator deserves its own blog post ideas:
1. Show graph from Garofolo
2. Look for coidliytv and carlos buenos vinos codes
3. Show the differences and explain why aggregators deserve a first class citizen.
4. Explain the different options, on ifnra mainly, pull or push? adv and disadv.
5. Link this text as workig exmaple, 
===

Let's deal with the aggregator. I will make an effort to leave for the infrastructure layer important
concerns, mainly the fact that it's gonna be a polling kind of subscriber and others. Will it be possible
to delegate that to the infrastructure and code inside our application the needed exchange?

Honestly, at this point I have no idea what is gonna happen, maybe it's not possible and the fact
that I am polling will become an application or domain concern. Will that be the case? let's find out
together.

## Development

One thing is clear, and it's that the end result in the infra I have in mind, is a "regular" SQL table
with regular rows.

It might be wrong to think in those terms, it's like when you are learning a new language and the teacher
doesn't allow you nor gives you the translation to your mother tongue, so get used to the meaning in the
new language, that might have not an exact translation to your language. 

On hexagonal architecture we want to develop independently of the infra, but let's do an exception here
to understand it better and realize that after all, we want to allow users to login, that is, find a 
user by the username, in our case email, and match the hashed password to the one we have saved.

So in the case of SQL, we need a table with email and credentials, and well, don't we need that in any case?
let's then fix that as a domain view, we will call it user_credentials_view, and it will have just
the email and the hashed password strings. See! We started with the domain after all!

We won't use value objects for the view, just plain types. Maybe we are stripping from domain meaning
but there's no need to validate its values at this point - for me the main use for value objects. If
we have invalid values here, well we did something very wrong on the Command side then.

Another reason to use plain types is that Rust is a very strongly typed language, and I have felt, during
the small time I've been developing in rust, that I was over -  engineering with value objects, so right
now it's just a personal preference. But there's nothing wrong to use value objects for the views, and report
any errors back to the infra if we find them. It's probably better.

So we have our view like this:
user_credentials_view.rs

```rust
struct UserCredentialsView{
    email: String,
    hashed_credentials: String,
}
```
Having simple types makes for this view basically an "empty" DTO. I think that's quite all right. No
harm in that.

Before continuing with the domain, let's switch our focus to the application part. What do we want
our aggregator to do/use the domain?

## The application layer

You might want to take a look at this post `link to post` about my opinion of aggregators. I think
they are full pieces, like a command handler. So an aggregator, belong naturally to the application
layer, and it will have all the pieces to execute the domain the way it will let us do it.

Create a user_credentials_aggregator.rs on the application layer and what do we want from it?
Soemthing like this:
1.let events = event_store.load_all_events(last_read_event, batch).await?;
2. for each event:
3.  check the event type is the one we are interested in.
4. generate the view from the info in that event.
5. save the view using a regular "SQL" repository.

The file at this stage will look like this:

```rust
#[tracing::instrument(
name = "User Credentials aggregator",
skip(event_store, view_repository)
)]
pub async fn user_credentials_aggregator(
    event_store:  impl EventStoreInterface<User>,
    view_repository: impl UserViewRepositoryInterface,
    last_read_event: i64
) -> Result<(), UserViewRepositoryError>  {
    let events = event_store.load_all_events(last_read_event).await?;
    for event in events {
        match event.payload {
            RegisteredUser(user_registered_domain_event) => {
                let user_credentials_view: UserCredentialsView = user_registered_domain_event.into();
                view_repository.save_view(user_credentials_view).await?;
            }
        }
    }
    Ok(())
}
```
The whole thing should be idempotent, in case we are processing the same events over and over
the end result should be the same. Let's just not forget about that. At the end we will review
if it's idempotent or not.

We are just missing the conversion from the event payload to the view. On user_credentials_view

```rust
impl From<UserRegisteredDomainEvent> for UserCredentialsView {
    fn from(event: UserRegisteredDomainEvent) -> Self {
        Self{
            email: event.email,
            hashed_credentials: event.password_hash
        }
    }
}
```

We can already guess that having views like that standalone might become in a boom of classes per view.

Let's just finish though, and we will come back later.


## The view model

Instead of having a class per view, we could just have a Read Model. Now it makes sense! We already
have a ViewRepositoryInterface that stores the views we generated from playing the events.

Let's just make this views a little bit less use case specific and more holistic.

## The neverending variations

The power that the message store allow us though is incredible:
We can have several aggregators, to be subscribed directly, to be executed by the cron
etc, Each one takes care of a view. We can set at what time each view should be refreshed.

I would say there are a lot of several variation:
subscribe to a specific kind of event.
push vs pull
subscribe to a whole stream.
subscribe to all the events 

## Dislaimer about versioning

I have based the Event Store implementation in the cqqrs-es create, but here it's obvious that
the stream == aggregate. As per Young book, this doesn;t need to be the case. Actually, for
versioning reason, it could be very comfortable to be able to create a couple of aggregates
from a single stream, or that an aggregate might need a couple of streams.

That doesn't match my implementation of ES. I would need to add a new column for stream
and many functions to get events by stream. Streams should
become a Domain concept probably, In this project, I would say maybe all the 
events would have the same stream called "identity". If I happen to have more aggregates in this
microservice, the aggregate column would be that new name, but the stream would be "identity".

Or If I miss designed something, with teh stream, I would be able to separate and create a new 
set of events for versioning, updating purposes.

. Probably this change oculd be done as an addition to what there is now, and everything
that exists should keep functioning as it is. I am just a noob, but I will try to do that.

## Polling vs Pushing
To have two streams, probably I would have the aggregators in a different machine. That;s
why I like that aggregates are polling, so you can connect and disconnect them from streams
and place them anywhere.

Pushing is instantly, I don't need to deal with subsctiption/cron, but the write model, the 
commands need to somehow execute the aggregators. It can still be independent, adding some 
kind of bus on teh command handlers, that will trigger the aggregators.

In that case, the aggregators wouldn't need to have access to the message store, just to the 
read model repository, since each event is being sent through the bus. TODO refactor then my code???
PROBABLY!, maybe things I put in the application should fall to infra??? so the same aggregator
could be used by both Pull and Push way of working??





