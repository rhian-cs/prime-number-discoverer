use chrono::{DateTime, Local};

pub struct PrimeNumber {
    pub number: u32,
    pub created_at: DateTime<Local>,
    pub elapsed_secs: f64,
}

impl PrimeNumber {
    pub fn new(number: u32, elapsed_secs: f64) -> Self {
        Self {
            number,
            elapsed_secs,
            created_at: Local::now(),
        }
    }
}
