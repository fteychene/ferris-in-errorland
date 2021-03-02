# Ferris in error land

This repository is a compendium of libraries that were used to manage error in Rust at some point in time.
The idea is to look at how handling error evolved in time and what's the current way to managed them in how programs.

The project is a copycat of the base project that manage errors.
This project load a file of attendees and pick at random some winner in the list and print them.

By default the implementation are meant to fail in a business way (ask more winenrs than exiting attendees) and on a technical way (Try to read from an inexistant file).

## Base

_Run_ : `cargo run --bin base`

This project define the base code that we want to manage errors from.
It will panic if you run it.  


# History

## Error-chain

:warning: Deprecated :warning:

_Run_ : `cargo run --bin error-chain`

This project use the library [error-chain](https://docs.rs/error-chain/0.12.4/error_chain/).
This lib generate all error management for you using a macro. it was create and used at a time where error was a real pain to extend and generate everything you need.

This lib is now deprecated and should not be used.

## Failure

:warning: Deprecated :warning:

_Run_ : `cargo run --bin failure`

Use of the lib [failure](https://rust-lang-nursery.github.io/failure/)

Interesting points :
 - Bypass std::error::Error trait limitation with Fail trait
 - Debug & Display for human comprehension
 - Composable backtrace & cause
 - Send + Sync => can be moved and shared between threads easily.
 - Derive fail to create custom errors type

# Current state

## Standard library

_Run_ : `cargo run --bin std`

 - [RFC](https://rust-lang.github.io/rfcs/2504-fix-error.html)
 - Display + Debug impls, for printing error messages.  
 - The backtrace method.
 - The source method. This returns another Error type, which is the underlying source of this error.

We can see that standard library error is now a good way to create a manage errors but it miss some utils that where provided by libraries like error-chain or failure.

## Snafu

_Run_ : `cargo run --bin snafu`

 - Ideology close to error-chain where all errors should be defined as a type that will wrap other error or business ones.
 - Add context to error with an easy wrapping mechanic
 - Simple to create and contextualize errors

## Thiserror + Anyhow

Thiserror alone _Run_ : `cargo run --bin thiserror`
Anyhow alone _Run_ : `cargo run --bin anyhow`
Combined _Run_ : `cargo run --bin diptych`

 - Create your custom error easily by using derive macro for std::error::Error
 - A better Box<dyn std::error::Error>
 - Work around standard std::error::Error but add functionalities

Currently I'm using the diptych anyhow / thiserror in my projects


# The future

See the error working group [here](https://rust-lang.github.io/rfcs/2965-project-error-handling.html)