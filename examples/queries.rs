use std::{error::Error, fs};

use chrono::Local;
use rusqlite::Connection;

const DATABASE_PATH: &str = "tmp/primes.db";

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DATABASE_PATH)?;

    fs::create_dir_all("tmp/")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS primes (
                number       INTEGER,
                created_at   TEXT,
                elapsed_secs REAL
            )",
        (),
    )?;

    conn.execute(
        "INSERT INTO primes
        (number, created_at, elapsed_ms)
        VALUES
        (?1, ?2, ?3)
        ",
        (
            &5,
            &Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            0.0,
        ),
    )?;

    const DEFAULT_PRIME: u64 = 2;

    let mut stmt = conn.prepare("SELECT MAX(number) FROM primes")?;
    let mut primes_iter = stmt.query_map([], |row| row.get::<_, u64>(0))?;
    let latest_prime = primes_iter.next().unwrap_or(Ok(DEFAULT_PRIME))?;

    println!("{latest_prime}");

    Ok(())
}
