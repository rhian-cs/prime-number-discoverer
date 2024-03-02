use std::{error::Error, fs, io, path::Path, time::Duration};

use chrono::{DateTime, Local};
use rusqlite::Connection;

const DATABASE_PATH: &str = "tmp/primes.db";

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open_and_setup() -> Result<Self, Box<dyn Error>> {
        create_parent_directory(DATABASE_PATH)?;

        let db = Self {
            conn: Connection::open(DATABASE_PATH)?,
        };

        db.conn.execute(
            "CREATE TABLE IF NOT EXISTS primes (
                number     INTEGER,
                created_at TEXT,
                elapsed_ms REAL
            )",
            (),
        )?;

        Ok(db)
    }

    pub fn insert_prime(
        &self,
        number: u32,
        created_at: DateTime<Local>,
        elapsed: Duration,
    ) -> Result<(), rusqlite::Error> {
        let formatted_created_at = created_at.format("%Y-%m-%d %H:%M:%S").to_string();

        self.conn.execute(
            "INSERT INTO primes
            (number, created_at, elapsed_ms)
            VALUES
            (?1, ?2, ?3)
            ",
            (&number, &formatted_created_at, elapsed.as_secs_f64()),
        )?;

        Ok(())
    }

    pub fn get_latest_prime(&self) -> Result<u32, rusqlite::Error> {
        const DEFAULT_PRIME: u32 = 2;

        let mut stmt = self.conn.prepare("SELECT MAX(number) FROM primes")?;
        let mut primes_iter = stmt.query_map([], |row| row.get::<_, u32>(0))?;
        let latest_prime = primes_iter.next().unwrap_or(Ok(DEFAULT_PRIME))?;

        Ok(latest_prime)
    }
}

fn create_parent_directory(path_str: &str) -> Result<(), io::Error> {
    let path = Path::new(path_str);
    let directory = path.parent().ok_or::<io::Error>(io::Error::new(
        io::ErrorKind::NotFound,
        format!("could not find parent for {}", path_str),
    ))?;

    fs::create_dir_all(directory)?;

    Ok(())
}
