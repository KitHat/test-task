# README

## How to launch?

### Running the app

There are two ways how to launch this app:

```
1. cargo run --features="<custom1|custom2>"
2. cargo build --features="<custom1|custom2>"
   cd target/debug
   ./sprout-therapy-test-task
```
It will start on port 8000 on localhost. In features you should specify a customs set that you want to enable. If you want to run just on base set of rules you can omit the `--features` flag.

**NB**: you should be on `nightly` version of Rust to compile it. You can override the Rust version only for this directory:
```
rustup override set nightly
```

### Running the tests

I have only made unit tests to verify that the logic works as expected. You can run the tests with this command:
```
cargo test
```
You can use features to run the test that are test custom behaviour

## Decisions and Solutions

There was a couple of decisions regarding implementation and tools to use.

### Runtime vs Compile-time customs selection

It is not really a decision but a choice of implementation variant. We could have set the customs during the runtime (through the query param on rest endpoint or some config file that is reloaded) or we can set the customs during the compile time using features. In a way how this task is stated there is no strong arguments in favor of any of this solution. However in my opinion selecting behaviour during the compile time is more of a Rust'ish way of programming and here are the reasons why: 

* you can store artifacts 
* it will work faster -- unnessecary code will be just compiled out.
* executable will be smaller for the same reason.
* it will be easier to recompile it as a shared object -- you will not need to define some complex way of passing the behaviour through C API.

However I still can not find this argument strong enough to convince everybody for compile-time behaviour selection and will name it as my personal passion.

### REST API framework

Rust frameworks for REST API are not very mature (in comparison with monstrous frameworks like Spring in Java). There are three more or less real options:

* [Rocket](https://rocket.rs/) -- the most mature framwork from this three. Has good documentation and easy to use. Downside -- compiles only on `nightly` version of Rust but htey promise that in the next release it will be fixed.
* [Actix](https://actix.rs/) -- framework that uses a lot of unsafe behaviour to speed things up. I find it a bit more complicated to use than Rocket so it was not my choice.
* [Tower Web](https://github.com/carllerche/tower-web) -- a new framework, has potential, but very immature. Also, its documentation is very poor -- I guess if you will take it into production you wil have to find some things right from source code and contribute new features to it.

If I need to choose a framework for the production REST API project with obligatory Rust usage, I would make a deeper exploration of Actix. Although, for some small project without big production usage I would prefer something with a more gradual learning curve.