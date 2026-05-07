use anyhow::Result;
use rusqlite::Connection;

pub mod migrations;
pub mod repository;

pub fn open_in_memory() -> Result<Connection> {
    Ok(Connection::open_in_memory()?)
}

pub fn run_migrations(conn: &Connection) -> Result<()> {
    let current_version: i32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    let tx = conn.unchecked_transaction()?;

    for (version, sql) in migrations::MIGRATIONS {
        if *version > current_version {
            tx.execute_batch(sql)?;
            tx.pragma_update(None, "user_version", version)?;
        }
    }

    tx.commit()?;
    Ok(())
}
