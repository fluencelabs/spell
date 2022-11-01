#![feature(try_blocks)]

pub mod script;
pub mod kv;
pub mod error;

extern crate core;

use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {
    kv::create_db();
}

