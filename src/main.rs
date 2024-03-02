use std::{error::Error, time::Instant};

use chrono::Local;
use database::Database;
use is_prime::is_prime;
use log::{debug, info};

mod database;
mod division_range;
mod is_prime;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    debug!("Execution started.");

    let db = Database::open_and_setup()?;
    let latest_prime = db.get_latest_prime()?;

    let mut prime_candidate = latest_prime;

    loop {
        debug!("Now checking if {prime_candidate} is a prime.");

        let start_time = Instant::now();

        if is_prime(prime_candidate) {
            let elapsed = start_time.elapsed();

            info!(
                "{prime_candidate} is a prime. Took {}",
                elapsed.as_secs_f64()
            );

            db.insert_prime(prime_candidate, Local::now(), elapsed)?;
        }

        prime_candidate += 1;
    }
}
