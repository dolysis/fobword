use rusqlite::{params, Connection, Result, Error};
struct db
{
    conn: Connection
}

impl db
{
    pub fn new() -> Result<db>
    {
        let conn = Connection::open(r"G:\Programming\Github\Dolyosis\cross-platform\testdb.db3")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS macros (
                    name TEXT NOT NULL PRIMARY KEY,
                    password TEXT NOT NULL)", 
            []
                )?;
        
        Ok(db { conn })
    }

    pub fn insert_macro(&self, name: &str, pass: &str) -> Result<usize>
    {
        self.conn.execute(
            "INSERT INTO macros (name, password) VALUES (?1, ?2)", 
        params![name, pass])
    }
}