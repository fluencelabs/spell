/*
 * Aqua Spell Service
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#![feature(try_blocks)]

#[macro_use]
extern crate fstrings;

use marine_rs_sdk::module_manifest;

pub mod auth;
pub mod error_handling;
pub mod kv;
pub mod log;
pub mod mailbox;
mod misc;
pub mod schema;
pub mod script;
pub mod trigger_config;

module_manifest!();

pub fn main() {
    schema::create();
}
