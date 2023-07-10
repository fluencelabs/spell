use fstrings::f;
use marine_sqlite_connector::Connection;

pub const DEFAULT_MAX_ERR_PARTICLES: usize = 50;
pub const DEFAULT_MAX_MAILBOX: usize = 50;
pub const DEFAULT_MAX_LOGS: usize = 50;
pub const DB_FILE: &'static str = "/tmp/spell.sqlite";

pub fn db() -> Connection {
    // use rand::prelude::*;
    //
    // let db_path = if std::path::Path::new("/tmp/this_is_test").exists() {
    //     format!("/tmp/{}_spell.sqlite", rand::random::<u32>())
    // } else {
    //     format!(DB_FILE)
    // };
    marine_sqlite_connector::open(DB_FILE).expect("open sqlite db")
}

pub fn create() {
    let conn = db();
    conn.execute(
        f!(r#"
            CREATE TABLE IF NOT EXISTS trigger_config (
                -- clock config
                start_sec INTEGER, end_sec INTEGER, period_sec INTEGER,
                -- connection pool config
                connect INTEGER, disconnect INTEGER,
                -- blockchain config
                start_block INTEGER, end_block INTEGER
            );

            CREATE TABLE IF NOT EXISTS relay (relay TEXT);

            -- CREATE TABLE IF NOT EXISTS kv (key TEXT, string TEXT, u32 INTEGER, list_index INTEGER);
            CREATE TABLE IF NOT EXISTS kv (
                key TEXT NOT NULL,
                string TEXT,
                u32 INTEGER,
                list_index INTEGER DEFAULT -1,

                PRIMARY KEY(key, list_index)
            );

            -- particles stored in the database, LRU-like
            CREATE TABLE IF NOT EXISTS particles (particle_id TEXT PRIMARY KEY, timestamp INTEGER);
            -- errors happened in particles
            CREATE TABLE IF NOT EXISTS errors (
                particle_id TEXT,
                timestamp INTEGER,
                error_idx INTEGER,
                error_code INTEGER,
                instruction TEXT,
                message TEXT,
                peer_id TEXT
            );
            CREATE TABLE IF NOT EXISTS config_table (parameter TEXT PRIMARY KEY, value INTEGER NOT NULL);
            -- maximum number of particles to store information about
            INSERT OR REPLACE INTO config_table VALUES ('max_particles', {DEFAULT_MAX_ERR_PARTICLES});
            -- current count of stored particles
            INSERT OR REPLACE INTO config_table VALUES ('count_particles', 0);
             -- maximum number of logs to store
            INSERT OR REPLACE INTO config_table VALUES ('max_logs', {DEFAULT_MAX_LOGS});
            -- current count of stored logs
            INSERT OR REPLACE INTO config_table VALUES ('count_logs', 0);
            -- maximum number of mailbox messages to store
            INSERT OR REPLACE INTO config_table VALUES ('max_mailbox', {DEFAULT_MAX_MAILBOX});
            -- current count of stored mailbox messages
            INSERT OR REPLACE INTO config_table VALUES ('count_mailbox', 0);

            -- if there are more than `max_particles` particles, delete the oldest one
            CREATE TRIGGER IF NOT EXISTS errors_limit_trigger AFTER INSERT ON particles
                FOR EACH ROW
                -- if limit is reached
                WHEN (SELECT value FROM config_table WHERE parameter = 'count_particles')
                    > (SELECT value FROM config_table WHERE parameter = 'max_particles')
                BEGIN
                    -- delete all errors for the oldest particle
                    DELETE FROM particles
                        -- take oldest by 'timestamp' column
                        WHERE particle_id = (SELECT particle_id FROM particles ORDER BY timestamp LIMIT 1);
                END;

            -- when a particle is removed, remove its errors
            CREATE TRIGGER IF NOT EXISTS clear_errors AFTER DELETE ON particles
                FOR EACH ROW
                BEGIN
                    -- remove all errors for that particle
                    DELETE FROM errors WHERE particle_id = OLD.particle_id;
                    -- decrement number of particles
                    UPDATE config_table SET value = value - 1 WHERE parameter = 'count_particles';
                END;

            -- when a particle is inserted, increment the counter
            CREATE TRIGGER IF NOT EXISTS particles_count_insert_trigger AFTER INSERT ON particles
                FOR EACH ROW
                BEGIN
                  UPDATE config_table SET value = value + 1 WHERE parameter = 'count_particles';
                END;

            -- when a particle error is inserted, store particle id if it wasn't there yet
            CREATE TRIGGER IF NOT EXISTS store_particle_id AFTER INSERT ON errors
                FOR EACH ROW
                BEGIN
                    INSERT OR IGNORE INTO particles (particle_id, timestamp) VALUES (NEW.particle_id, NEW.timestamp);
                END;

            CREATE TABLE IF NOT EXISTS logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                log TEXT
            );

            CREATE TRIGGER IF NOT EXISTS logs_insert_and_limit_trigger AFTER INSERT ON logs
                FOR EACH ROW
                BEGIN
                    -- when a log is inserted, increment the counter
                    UPDATE config_table SET value = value + 1 WHERE parameter = 'count_logs';

                    -- if there are more than `max_logs` logs, delete the oldest ones
                    DELETE FROM logs
                    WHERE (SELECT value FROM config_table WHERE parameter = 'count_logs')
                        > (SELECT value FROM config_table WHERE parameter = 'max_logs')
                    AND id = (SELECT id FROM logs ORDER BY timestamp LIMIT 1);

                    -- decrement number of logs
                    UPDATE config_table SET value = value - 1 WHERE parameter = 'count_logs'
                    AND (SELECT value FROM config_table WHERE parameter = 'count_logs')
                        > (SELECT value FROM config_table WHERE parameter = 'max_logs');
                END;

            CREATE TABLE IF NOT EXISTS mailbox (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                message TEXT
            );

            CREATE TRIGGER IF NOT EXISTS mailbox_insert_and_limit_trigger AFTER INSERT ON mailbox
                FOR EACH ROW
                BEGIN
                  -- when a mailbox message is inserted, increment the counter
                  UPDATE config_table SET value = value + 1 WHERE parameter = 'count_mailbox';

                  -- if there are more than `max_mailbox` messages, delete the oldest ones
                  DELETE FROM mailbox
                  WHERE (SELECT value FROM config_table WHERE parameter = 'count_mailbox')
                        > (SELECT value FROM config_table WHERE parameter = 'max_mailbox')
                  AND id = (SELECT id FROM mailbox ORDER BY timestamp LIMIT 1);

                  -- decrement number of mailbox messages
                  UPDATE config_table SET value = value - 1 WHERE parameter = 'count_mailbox'
                  AND (SELECT value FROM config_table WHERE parameter = 'count_mailbox')
                        > (SELECT value FROM config_table WHERE parameter = 'max_mailbox');
                END;

            "#),
    )
        .expect("init sqlite db");
}
