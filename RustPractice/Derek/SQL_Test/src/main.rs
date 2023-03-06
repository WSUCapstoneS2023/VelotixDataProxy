use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // Open a connection to a new or existing SQLite database file
    let conn = Connection::open("example.db")?;

    // Create a new table named `users`
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  email           TEXT NOT NULL
                  )",
        [],
    )?;

    Ok(())
}