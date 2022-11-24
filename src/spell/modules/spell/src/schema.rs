use fstrings::f;
use marine_sqlite_connector::Connection;

pub const DEFAULT_MAX_ERR_PARTICLES: usize = 50;

pub fn db() -> Connection {
    // use rand::prelude::*;
    //
    // let db_path = if std::path::Path::new("/tmp/this_is_test").exists() {
    //     format!("/tmp/{}_spell.sqlite", rand::random::<u32>())
    // } else {
    //     format!("/tmp/spell.sqlite")
    // };
    marine_sqlite_connector::open("/tmp/spell.sqlite").expect("open sqlite db")
}

pub fn create() {
    db().execute(
        f!(r#"
            CREATE TABLE IF NOT EXISTS relay (relay TEXT);

            CREATE TABLE IF NOT EXISTS kv (key TEXT, string TEXT, u32 INTEGER, list_index INTEGER);

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
            CREATE TABLE IF NOT EXISTS particle_count (parameter TEXT PRIMARY KEY, value INTEGER NOT NULL);
            -- maximum number of particles to store information about
            INSERT OR REPLACE INTO particle_count VALUES ('max_particles', {DEFAULT_MAX_ERR_PARTICLES});
            -- current count of stored particles
            INSERT OR REPLACE INTO particle_count VALUES ('count_particles', 0);


            -- if there are more than `max_particles` particles, delete the oldest one
            CREATE TRIGGER IF NOT EXISTS errors_limit_trigger AFTER INSERT ON particles
                FOR EACH ROW
                -- if limit is reached
                WHEN (SELECT value FROM particle_count WHERE parameter = 'count_particles')
                    > (SELECT value FROM particle_count WHERE parameter = 'max_particles')
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
                    UPDATE particle_count SET value = value - 1 WHERE parameter = 'count_particles';
                END;

            -- when a particle is inserted, incremenet the counter
            CREATE TRIGGER IF NOT EXISTS particles_count_insert_trigger AFTER INSERT ON particles
                FOR EACH ROW
                BEGIN
                  UPDATE particle_count SET value = value + 1 WHERE parameter = 'count_particles';
                END;

            -- when a particle error is inserted, store particle id if it wasn't there yet
            CREATE TRIGGER IF NOT EXISTS store_particle_id AFTER INSERT ON errors
                FOR EACH ROW
                BEGIN
                    INSERT OR IGNORE INTO particles (particle_id, timestamp) VALUES (NEW.particle_id, NEW.timestamp);
                END;
            "#),
    )
        .expect("init sqlite db");
}
