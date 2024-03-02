use std::{error::Error, fs, io, path::Path};

use log::debug;
use rusqlite::Connection;

use crate::prime_number::PrimeNumber;

const DATABASE_PATH: &str = "tmp/primes.db";

pub struct Database {
    conn: Connection,
    delayed_primes: Vec<PrimeNumber>,
}

impl Database {
    pub fn setup() -> Result<Self, Box<dyn Error>> {
        create_parent_directory(DATABASE_PATH)?;

        let db = Self {
            conn: Connection::open(DATABASE_PATH)?,
            delayed_primes: Vec::new(),
        };

        db.conn.execute(
            "CREATE TABLE IF NOT EXISTS primes (
                number       INTEGER,
                created_at   TEXT,
                elapsed_secs REAL
            )",
            (),
        )?;

        Ok(db)
    }

    pub fn add_prime(&mut self, prime: PrimeNumber) -> Result<(), rusqlite::Error> {
        self.delayed_primes.push(prime);

        if self.delayed_primes.len() >= 10000 {
            self.flush_primes()?;
        }

        Ok(())
    }

    pub fn flush_primes(&mut self) -> Result<(), rusqlite::Error> {
        debug!("Now flushing prime numbers to disk.");

        let tx = self.conn.transaction()?;

        for prime in self.delayed_primes.iter() {
            let formatted_created_at = prime.created_at.format("%Y-%m-%d %H:%M:%S").to_string();

            tx.execute(
                "INSERT INTO primes
                (number, created_at, elapsed_secs)
                VALUES
                (?1, ?2, ?3)",
                (prime.number, formatted_created_at, prime.elapsed_secs),
            )?;
        }

        tx.commit()?;

        self.delayed_primes = Vec::new();

        Ok(())
    }

    pub fn get_latest_prime(&self) -> Result<u32, rusqlite::Error> {
        const DEFAULT_PRIME: u32 = 2;

        let mut stmt = self.conn.prepare("SELECT MAX(number) FROM primes")?;
        let mut primes_iter = stmt.query_map([], |row| row.get::<_, u32>(0))?;
        let latest_prime = primes_iter
            .next()
            .unwrap_or(Ok(DEFAULT_PRIME))
            .unwrap_or(DEFAULT_PRIME);

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
