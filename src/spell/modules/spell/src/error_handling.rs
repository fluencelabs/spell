use std::convert::TryFrom;

use eyre::WrapErr;
use marine_rs_sdk::marine;
use marine_sqlite_connector::{State, Statement};

use crate::result::UnitResult;
use crate::schema::db;

/// The `%last_error%` content.
#[marine]
#[derive(Clone, Debug)]
pub struct LastError {
    /// The error code.
    pub error_code: u32,
    /// The failed instruction.
    pub instruction: String,
    /// The error message.
    pub message: String,
    /// The peer where the call failed.
    pub peer_id: String,
}

#[marine]
#[derive(Clone, Debug)]
pub struct LastErrorEntry {
    /// The reported error.
    pub last_error: LastError,
    /// The number of the error assigned in the script itself
    pub error_idx: u32,
}

impl TryFrom<&mut Statement> for LastErrorEntry {
    type Error = marine_sqlite_connector::Error;

    fn try_from(statement: &mut Statement) -> Result<Self, Self::Error> {
        Ok(Self {
            error_idx: statement.read::<f64>(0)? as u32,
            last_error: LastError {
                error_code: statement.read::<f64>(1)? as u32,
                instruction: statement.read(2)?,
                message: statement.read(3)?,
                peer_id: statement.read(4)?,
            },
        })
    }
}

#[marine]
#[derive(Clone, Debug)]
pub struct ParticleErrors {
    particle_id: String,
    errors: Vec<LastErrorEntry>,
}

#[marine]
pub fn store_error(error: LastError, particle_timestamp: u64, error_idx: u32) -> UnitResult {
    let call_parameters = marine_rs_sdk::get_call_parameters();

    let result: eyre::Result<()> = try {
        let mut statement = db().prepare(
            r#"
        INSERT INTO errors
            (particle_id, timestamp, error_idx, error_code, instruction, message, peer_id)
        VALUES
            (?, ?, ?, ?, ?, ?, ?)
        "#,
        )?;
        statement.bind(1, call_parameters.particle_id.as_str())?;
        statement.bind(2, particle_timestamp as f64)?;
        statement.bind(3, error_idx as f64)?;
        statement.bind(4, error.error_code as f64)?;
        statement.bind(5, error.instruction.as_str())?;
        statement.bind(6, error.message.as_str())?;
        statement.bind(7, error.peer_id.as_str())?;

        statement.next()?;
    };

    match result {
        Ok(_) => UnitResult::ok(),
        Err(e) => UnitResult::error(format!("Error storing error: {}", e)),
    }
}

#[marine]
pub fn get_errors(particle_id: String) -> Vec<LastErrorEntry> {
    let result: eyre::Result<Vec<LastErrorEntry>> = try {
        let mut statement = db().prepare(
            r#"
            SELECT
                (error_idx, error_code, instruction, message, peer_id)
            FROM
                errors WHERE particle_id = ?
        "#,
        )?;
        statement.bind(1, particle_id.as_str())?;
        std::iter::from_fn(move || {
            let r: eyre::Result<Option<LastErrorEntry>> = try {
                if let State::Row = statement.next()? {
                    Some(LastErrorEntry::try_from(&mut statement)?)
                } else {
                    None
                }
            };
            r.context("error fetching error row from sqlite")
                .transpose()
        })
        .filter_map(|r| r.ok())
        .collect()
    };

    result.unwrap_or_default()
}

#[marine]
pub fn get_all_errors() -> Vec<ParticleErrors> {
    // let result: eyre::Result<Vec<LastErrorEntry>> = try {
    //     let mut statement = db().prepare(
    //         r#"
    //         SELECT
    //             (error_idx, error_code, instruction, message, peer_id, particle_id)
    //         FROM
    //             errors
    //     "#,
    //     )?;
    //     std::iter::from_fn(move || {
    //         let r: eyre::Result<Option<(String, LastErrorEntry)>> = try {
    //             if let State::Row = statement.next()? {
    //                 let err = LastErrorEntry::try_from(&mut statement)?;
    //                 let particle_id = statement.read::<String>(5)?;
    //                 Some((particle_id, err))
    //             } else {
    //                 None
    //             }
    //         };
    //         r.context("error fetching error row from sqlite")
    //             .transpose()
    //     })
    //     .filter_map(|r| r.ok())
    //     .group_by(|(pid, _)| pid)
    //     .map(|(particle_id, errors)|
    //         ParticleErrors {
    //             particle_id,
    //             errors: errors.into_iter().map(|(_, err)| err).collect()
    //         })
    //     .collect()
    // };
    //
    // result.unwrap_or_default()

    todo!()
}
