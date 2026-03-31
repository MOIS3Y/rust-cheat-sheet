//! Time handling operations.
//!
//! Demonstrates common methods from `std::time`:
//! `Duration`, `Instant`, and `SystemTime`.

use std::time::{Duration, Instant, SystemTime};

/// Create Duration from components.
///
/// `Duration::new(secs, nanos)` creates a duration
/// from seconds and nanoseconds.
pub fn duration_new() {
    let duration = Duration::new(5, 0);
    assert_eq!(duration.as_secs(), 5);
    assert_eq!(duration.subsec_nanos(), 0);
}

/// Create Duration from milliseconds.
///
/// `Duration::from_millis()` creates a duration
/// from milliseconds.
pub fn duration_from_millis() {
    let from_millis = Duration::from_millis(1500);
    assert_eq!(from_millis.as_secs(), 1);
    assert_eq!(from_millis.subsec_millis(), 500);
}

/// Measure elapsed time with Instant.
///
/// `Instant::now()` captures the current instant.
/// `elapsed()` returns duration since then.
pub fn instant_elapsed() {
    let start = Instant::now();
    std::thread::sleep(Duration::from_millis(10));
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() >= 10);
}

/// Get system time since Unix epoch.
///
/// `SystemTime::now()` returns current system time.
/// `duration_since()` calculates duration from a point.
pub fn system_time_epoch() {
    let now = SystemTime::now();
    let duration_since_unix = now.duration_since(SystemTime::UNIX_EPOCH);

    match duration_since_unix {
        Ok(duration) => {
            let _secs = duration.as_secs();
        }
        Err(_) => {
            panic!("System time is before Unix epoch!");
        }
    }
}

/// Safe time arithmetic with checked_add.
///
/// `checked_add()` returns `None` on overflow.
/// Safer than `add()` which panics on overflow.
pub fn checked_add_time() {
    let now = SystemTime::now();
    let later = now.checked_add(Duration::from_secs(3600));
    assert!(later.is_some());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_new() {
        duration_new();
    }

    #[test]
    fn test_duration_from_millis() {
        duration_from_millis();
    }

    #[test]
    fn test_instant_elapsed() {
        instant_elapsed();
    }

    #[test]
    fn test_system_time_epoch() {
        system_time_epoch();
    }

    #[test]
    fn test_checked_add_time() {
        checked_add_time();
    }
}
