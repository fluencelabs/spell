#![feature(try_blocks)]

pub mod auth;
pub mod error;
pub mod kv;
pub mod result;
pub mod script;

extern crate core;

use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {
    kv::create_db();
}
