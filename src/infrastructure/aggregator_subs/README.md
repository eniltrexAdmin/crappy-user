# The aggregator

The idea of the aggregator is 

## Development

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

Before continuing with the domain, let's switch our focus to the application part. What do we want
our aggregator to do/use the domain?

## The application layer

I have decided that aggregators are like command handlers, therefore belong to the application layer.