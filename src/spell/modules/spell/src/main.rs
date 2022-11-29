#![feature(try_blocks)]

extern crate core;
#[macro_use]
extern crate fstrings;

use marine_rs_sdk::module_manifest;

pub mod auth;
pub mod collection_kv;
pub mod error;
pub mod error_handling;
pub mod kv;
pub mod location;
pub mod schema;
pub mod script;
pub mod trigger_config;
pub mod value;

module_manifest!();

pub fn main() {
    schema::create();
}
