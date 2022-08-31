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

You might want to take a look at this post `link to post` about my opinion of aggregagtors. I think
they are full pieces, like a command handler. So an aggregator, belong naturaly to the application
layer, and it will have all the pieces to execute the domain the way it will let us do it.

Create a user_credentials_aggregator.rs on the application layer and what do we want from it?

```rust
pub async fn user_credentials_aggregator(
// something we don't know yet.
){
    // we want to get all the events for the user aggregate:
    let events = event_store.load_all_events(last_read_event, batch).await?;
    for event in events {
        // check the event type is the one we are interested in.
        // generate the view from the info in that event.
        // save the view using a regular "SQL" repository.
    }
}
```
The whole thing should be idempotent, in case we are processing the same events over and over
the end result should be the same. Let's just not forget about that. At the end we will review
if it's idempotent or not.
