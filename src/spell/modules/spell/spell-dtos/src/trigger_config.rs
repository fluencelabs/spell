/*
 * Copyright 2024 Fluence DAO
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::value::SpellValueT;
use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};

#[marine]
#[derive(Default, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TriggerConfigValue {
    pub config: TriggerConfig,
    pub success: bool,
    pub error: String,
}

impl SpellValueT for TriggerConfigValue {
    fn is_success(&self) -> bool {
        self.success
    }

    fn take_error(self) -> String {
        self.error
    }
}

#[marine]
#[derive(Default, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TriggerConfig {
    /// Trigger spell by clock
    pub clock: ClockConfig,
    /// Trigger spell on connect/disconnect events
    pub connections: ConnectionPoolConfig,
    /// Trigger spell on blockchain blocks
    pub blockchain: BlockChainConfig,
}

#[marine]
#[derive(Default, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ClockConfig {
    /// Defines when to start trigger spell.
    /// Unix time. 0 means 'do not run'
    pub start_sec: u32,
    /// Defines when to stop trigger spell. Will not trigger after that timestamp.
    /// Unix time. 0 means 'never stop'
    pub end_sec: u32,
    /// Defines how often to trigger spell
    /// 0 means 'do not subscribe'
    /// NOTE: Subject to host clock resolution limitations.
    ///       If small period is set, host may override it to a bigger one
    pub period_sec: u32,
}

#[marine]
#[derive(Default, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockChainConfig {
    /// Defines since what block to start trigger spell
    /// 0 means 'do not subscribe'
    /// TODO: what about blocks in the past? will host replay them?
    pub start_block: u32,
    /// Defines until what block to keep trigger spell. Will not trigger after that block.
    /// 0 means 'never stop'
    pub end_block: u32,
}

#[marine]
#[derive(Default, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ConnectionPoolConfig {
    /// Defines whether to trigger spell on connect events
    pub connect: bool,
    /// Defines whether to trigger spell on disconnect events
    pub disconnect: bool,
}
