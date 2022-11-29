use marine_rs_sdk::marine;
use marine_sqlite_connector::State;

use crate::auth::is_by_creator;
use crate::error::SpellError::{NoTriggerConfig, SetTriggerConfigForbidden};
use crate::schema::db;
use crate::value::{format_error, UnitValue};

#[marine]
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct TriggerConfigValue {
    pub config: TriggerConfig,
    pub success: bool,
    pub error: String,
}

#[marine]
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct TriggerConfig {
    /// Trigger spell by clock
    pub clock: ClockConfig,
    /// Trigger spell on connect/disconnect events
    pub connections: ConnectionPoolConfig,
    /// Trigger spell on blockchain blocks
    pub blockchain: BlockChainConfig,
}

#[marine]
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ClockConfig {
    /// Defines when to start trigger spell.
    /// Unix time. 0 means 'do not subscribe'
    pub start_sec: u32,
    /// Defines when to stop trigger spell. Will not trigger after that timestamp.
    /// Unix time. 0 means 'never stop'
    pub end_sec: u32,
    /// Defines how often to trigger spell
    /// NOTE: Subject to host clock resolution limitations.
    ///       If small period is set, host may override it to a bigger one
    pub period_sec: u32,
}

#[marine]
#[derive(Default, Clone, Debug, Eq, PartialEq)]
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
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ConnectionPoolConfig {
    /// Defines whether to trigger spell on connect events
    pub connect: bool,
    /// Defines whether to trigger spell on disconnect events
    pub disconnect: bool,
}

#[marine]
pub fn set_trigger_config(config: TriggerConfig) -> UnitValue {
    if !is_by_creator() {
        return SetTriggerConfigForbidden.into();
    }

    let result: eyre::Result<()> = try {
        db().execute("DELETE FROM trigger_config")?;

        let mut statement = db().prepare(
            r#"
            INSERT INTO trigger_config (
                start_sec, end_sec, period_sec,
                connect, disconnect,
                start_block, end_block
            ) VALUES (
                ?, ?, ?,
                ?, ?,
                ?, ?
            )"#,
        )?;

        let TriggerConfig {
            clock,
            connections,
            blockchain,
        } = config;
        statement.bind(1, clock.start_sec as f64)?;
        statement.bind(2, clock.end_sec as f64)?;
        statement.bind(3, clock.period_sec as f64)?;
        statement.bind(4, connections.connect as u32 as f64)?;
        statement.bind(5, connections.disconnect as u32 as f64)?;
        statement.bind(6, blockchain.start_block as f64)?;
        statement.bind(7, blockchain.end_block as f64)?;
        statement.next()?;
    };

    result.into()
}

#[marine]
pub fn get_trigger_config() -> TriggerConfigValue {
    let result: eyre::Result<TriggerConfig> = try {
        let mut statement = db().prepare(r#"SELECT * FROM trigger_config"#)?;
        if let State::Row = statement.next()? {
            let start_sec = statement.read::<f64>(0)? as u32;
            let end_sec = statement.read::<f64>(1)? as u32;
            let period_sec = statement.read::<f64>(2)? as u32;
            let connect = statement.read::<f64>(3)? != 0f64;
            let disconnect = statement.read::<f64>(4)? != 0f64;
            let start_block = statement.read::<f64>(5)? as u32;
            let end_block = statement.read::<f64>(6)? as u32;

            TriggerConfig {
                clock: ClockConfig {
                    start_sec,
                    end_sec,
                    period_sec,
                },
                connections: ConnectionPoolConfig {
                    connect,
                    disconnect,
                },
                blockchain: BlockChainConfig {
                    start_block,
                    end_block,
                },
            }
        } else {
            return Err(NoTriggerConfig)?;
        }
    };

    match result {
        Ok(config) => TriggerConfigValue {
            config,
            success: true,
            error: <_>::default(),
        },
        Err(err) => TriggerConfigValue {
            config: <_>::default(),
            success: false,
            error: format_error(err),
        },
    }
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[ctor::ctor]
    /// usage of 'ctor' makes this function run only once
    fn before_all_tests() {
        std::fs::remove_file("/tmp/spell.sqlite").ok();
    }

    /// after_each macro copy-pastes this function into every test
    fn after_each() {
        std::fs::remove_file("/tmp/spell.sqlite").ok();
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn get_no_trigger_config(spell: marine_test_env::spell::ModuleInterface) {
        let get = spell.get_trigger_config();
        assert_eq!(
            get.success, false,
            "get succeeded without trigger config! wrong!"
        );
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn set_empty_trigger_config(spell: marine_test_env::spell::ModuleInterface) {
        let empty_config: crate::trigger_config::TriggerConfig = <_>::default();
        let empty_config: marine_test_env::spell::TriggerConfig =
            unsafe { std::mem::transmute(empty_config) };

        let set = spell.set_trigger_config(empty_config);
        assert!(set.success, "set empty config failed: {}", set.error);

        let get = spell.get_trigger_config();
        assert!(get.success, "get empty config failed: {}", get.error);

        let empty_config: crate::trigger_config::TriggerConfig = <_>::default();
        let config: crate::trigger_config::TriggerConfig =
            unsafe { std::mem::transmute(get.config) };
        assert_eq!(config, empty_config);
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn set_trigger_config(spell: marine_test_env::spell::ModuleInterface) {
        use marine_test_env::spell::{
            BlockChainConfig, ClockConfig, ConnectionPoolConfig, TriggerConfig,
        };

        let config = TriggerConfig {
            clock: ClockConfig {
                start_sec: 100,
                end_sec: 101,
                period_sec: 102,
            },
            connections: ConnectionPoolConfig {
                connect: true,
                disconnect: true,
            },
            blockchain: BlockChainConfig {
                start_block: 777,
                end_block: 999,
            },
        };

        let set = spell.set_trigger_config(config.clone());
        assert!(set.success, "set config failed: {}", set.error);

        let get = spell.get_trigger_config();
        assert!(get.success, "get config failed: {}", get.error);

        let expected_config: crate::trigger_config::TriggerConfig =
            unsafe { std::mem::transmute(config) };
        let loaded_config: crate::trigger_config::TriggerConfig =
            unsafe { std::mem::transmute(get.config) };
        assert_eq!(expected_config, loaded_config);
    }
}
