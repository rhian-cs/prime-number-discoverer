use std::{env, error::Error, sync::mpsc::channel, time::Instant};

use database::Database;
use is_prime::is_prime;
use log::{debug, info};

use crate::prime_number::PrimeNumber;

mod database;
mod division_range;
mod is_prime;
mod prime_number;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger()?;
    debug!("Execution started.");

    let mut db = Database::setup()?;

    let latest_prime = db.get_latest_prime()?;

    let mut prime_candidate = latest_prime;

    loop {
        debug!("Now checking if {prime_candidate} is a prime.");

        let start_time = Instant::now();

        if is_prime(prime_candidate) {
            let elapsed_secs = start_time.elapsed().as_secs_f64();

            info!("{prime_candidate} is a prime. Took {elapsed_secs}");

            db.add_prime(PrimeNumber::new(prime_candidate, elapsed_secs))?;
        }

        prime_candidate += 1;
    }
}

fn setup_logger() -> Result<(), Box<dyn Error>> {
    if env::var("RUST_LOG").unwrap_or("".into()).is_empty() {
        env::set_var("RUST_LOG", "trace");
    }

    env_logger::init();

    Ok(())
}
