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

use eyre::WrapErr;
use marine_sqlite_connector::State;
use marine_sqlite_connector::Statement;
use std::iter::from_fn;

pub fn fetch_rows<T, F>(mut statement: Statement, row_to_item: F) -> Vec<T>
where
    F: Fn(&mut Statement) -> eyre::Result<Option<T>>,
{
    from_fn(move || {
        let r: eyre::Result<Option<T>> = try {
            if State::Row == statement.next()? {
                row_to_item(&mut statement)?
            } else {
                None
            }
        };
        r.context("error fetching row from sqlite").transpose()
    })
    .filter_map(|r| r.ok())
    .collect()
}
