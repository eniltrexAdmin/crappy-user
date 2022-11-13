# Some comments

About mocks. I hate them.

I can't use dependencies on "dev" because of course, I want to mock
things that are not in tests but in Domain etc!!!

So in Cargo I can't put the mockall craete on dependencies-dev but on dependencies.

then, the macro  "automock" is Okis, but if I can't use it, then I need
to put my whoel trait/struct under macro! which is ugly as fuck, considering
that I am changing the code just for tests!!!

```rust
mock!{ 
 struct USer{}
}
```

So from now on, I';ll restruict highlt tests that  need mocks.

I'll go all the way to integration tests instead of unit test with mocks.
And just some interfaces should have that mock. I can't wrap my head around it!.


2022-11-13

I need to actually test it now at this stage.
I have the feeling there is something wrong on getting the hash/ encoded not encdoed.

I am not sure I will be able to authenticate for real.

And check teh aggreagator. start implementing the real complete Read Model

## Todo dynamic

- [ ] Play the app, see if it works
- [ ] Start implementing the real read model, the aggreagtor, and use that.
- [ ] The above should be enough for me to try to deploy to some prod. Create pipeline and so on.
- [ ] The rest of development with the events I imagined I needed. (see the read model to know more about what
I though could be interesting events.)
