use std::{error::Error, time::Instant};

use chrono::Local;
use database::Database;
use is_prime::is_prime;

mod database;
mod division_range;
mod is_prime;

fn main() -> Result<(), Box<dyn Error>> {
    let db = Database::open_and_setup()?;
    let latest_prime = db.get_latest_prime()?;

    let mut prime_candidate = latest_prime;

    loop {
        let start_time = Instant::now();

        if is_prime(prime_candidate) {
            db.insert_prime(prime_candidate, Local::now(), start_time.elapsed())?;
        }

        prime_candidate += 1;
    }
}
