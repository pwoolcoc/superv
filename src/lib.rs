//! # Supervisor tree library for Rust
//!
//! This library contains building blocks for creating supervisor trees
//! for supervising tasks, a la Erlang/OTP
//!

#![crate_type = "lib"]

#![feature(phase, globs)]

#[phase(plugin)] extern crate rest_easy;

pub mod supervisor;
// mod server;
// mod event;
// mod fsm;
