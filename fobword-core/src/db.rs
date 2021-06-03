use std::path::Path;

use rusqlite::{params, Connection, Result};
use super::command::Command;
/// Helper struct that holds the database connection and common operations
#[derive(Debug)]
pub struct Db
{
    conn: Connection
}

impl Db
{
    /// Create a new database connection from the path
    pub fn new(path: &Path) -> Result<Db>
    {
        let conn = Connection::open(path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS macros (
                    name TEXT NOT NULL PRIMARY KEY,
                    password TEXT NOT NULL)", 
            []
                )?;
        
        Ok(Db { conn })
    }

    /// Insert a macro into the database with the given name and password
    pub fn insert_macro(&self, name: &str, pass: &str) -> Result<usize>
    {
        self.conn.execute(
            "INSERT INTO macros (name, password) VALUES (?1, ?2)", 
        params![name, pass])
    }

    /// Delete a macro from the database
    pub fn delete_macro(&self, name: &str) -> Result<usize>
    {
        self.conn.execute(
            "DELETE FROM macros WHERE name = ?1",
            params![name]
        )
    }

    /// Update the password of a macro
    pub fn update_macro(&self, name: &str, pass: &str) -> Result<usize>
    {
        self.conn.execute(
            "UPDATE macros SET password = ?1 WHERE name = ?2", 
        params![pass, name]
        )   
    }

    /// Load all macros from the database
    pub fn load_macros(&self) -> Result<Vec<Command>>
    {
        let mut stmt = self.conn.prepare("SELECT name, password FROM macros")?;
        let mut rows = stmt.query([])?;
        let mut commands = Vec::new();
        while let Some(row) = rows.next()?
        {
            let name: String = row.get(0)?;
            let password: String = row.get(1)?;
            commands.push( Command { name, password});
        }
        Ok(commands)
    }
}