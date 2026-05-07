use anyhow::Result;
use rusqlite::Connection;

pub mod migrations;
pub mod repository;

pub fn open_in_memory() -> Result<Connection> {
    Ok(Connection::open_in_memory()?)
}

pub fn run_migrations(conn: &Connection) -> Result<()> {
    for sql in migrations::MIGRATIONS {
        conn.execute_batch(sql)?;
    }
    Ok(())
}
