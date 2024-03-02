use std::{
    ops::Range,
    sync::{Arc, Mutex},
    thread,
};

use crate::division_range::partitioned_division_ranges;

const MAX_THREADS: u32 = 12;

#[no_mangle]
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    let is_prime_assertion = Arc::new(Mutex::new(true));
    let mut handles = Vec::new();

    for range in partitioned_division_ranges(n, MAX_THREADS) {
        let is_prime_assertion = is_prime_assertion.clone();

        handles.push(thread::spawn(move || {
            compute_prime_assertion(n, range, is_prime_assertion)
        }));
    }

    handles.into_iter().for_each(|h| {
        h.join().unwrap();
    });

    let is_prime_assertion = is_prime_assertion.lock().unwrap();
    *is_prime_assertion
}

fn compute_prime_assertion(n: u32, range: Range<u32>, is_prime_assertion: Arc<Mutex<bool>>) {
    for divisor in range {
        if divisible_by(n, divisor) {
            let mut is_prime_assertion = is_prime_assertion.lock().unwrap();

            *is_prime_assertion = false;

            return;
        }
    }
}

fn divisible_by(a: u32, b: u32) -> bool {
    a % b == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    mod is_prime_fn {
        use super::*;

        #[test]
        fn it_validates_primes() {
            assert!(!is_prime(1));
            assert!(is_prime(2));
            assert!(is_prime(3));
            assert!(!is_prime(4));
            assert!(is_prime(5));
            assert!(!is_prime(6));
            assert!(is_prime(7));
            assert!(!is_prime(8));
            assert!(!is_prime(9));
            assert!(!is_prime(10));
        }

        #[test]
        #[ignore]
        fn it_validates_large_prime() {
            // Single threaded: Should take around 6 seconds
            // Multi threaded: Should take around 1 second
            assert!(is_prime(2_147_483_647));
        }

        #[test]
        #[ignore]
        fn it_rejects_large_non_prime() {
            assert!(!is_prime(1093 * 1321 * 2767));
        }
    }
}
