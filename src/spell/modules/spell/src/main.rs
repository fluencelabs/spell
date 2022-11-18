#![feature(try_blocks)]

#[macro_use]
extern crate fstrings;

pub mod auth;
pub mod error;
pub mod error_handling;
pub mod kv;
pub mod result;
pub mod schema;
pub mod script;

extern crate core;

use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {
    schema::create();
}
