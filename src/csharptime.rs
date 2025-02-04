/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */
/// This module for converting time from blueprint(c#) time to rust NaiveDateTime and vice versa
use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};

/// Returns C# epoch – 0001-01-01 00:00:00.
fn csharp_epoch() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(1, 1, 1)
        .expect("Invalid date")
        .and_hms_opt(0, 0, 0)
        .unwrap()
}

/// Converts the number of C# “ticks” (1 tick = 100 nanoseconds) to NaiveDateTime.
/// The function returns the time: epoch + number of seconds and microseconds calculated from ticks.
pub fn csharp_to_datetime(csharp_ticks: i64) -> NaiveDateTime {
    let seconds = csharp_ticks / 10_000_000;
    let residual = csharp_ticks % 10_000_000;
    // 1 tick = 100 nanoseconds, а 1 microsecond = 10 ticks.
    let microseconds = residual / 10;
    csharp_epoch() + Duration::seconds(seconds) + Duration::microseconds(microseconds)
}

/// Converts NaiveDateTime to C# tix number.
/// Fractional seconds (microseconds) are discarded here.
pub fn datetime_to_csharp(dt: NaiveDateTime) -> i64 {
    let duration = dt.signed_duration_since(csharp_epoch());
    duration.num_seconds() * 10_000_000
}

/// Returns the current time (UTC) as the number of C# ticks.
pub fn csharp_now() -> i64 {
    datetime_to_csharp(Utc::now().naive_utc())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_epoch() {
        let dt = csharp_to_datetime(0);
        assert_eq!(dt, csharp_epoch());
    }

    #[test]
    fn test_one_second() {
        let dt = csharp_to_datetime(10_000_000);
        let expected = csharp_epoch() + Duration::seconds(1);
        assert_eq!(dt, expected);
    }

    #[test]
    fn test_round_trip() {
        let dt = NaiveDate::from_ymd_opt(2020, 5, 20)
            .unwrap()
            .and_hms_opt(15, 30, 45)
            .unwrap();
        let ticks = datetime_to_csharp(dt);
        let dt_converted = csharp_to_datetime(ticks);
        assert_eq!(dt, dt_converted);
    }

    #[test]
    fn test_csharp_now() {
        let ticks = csharp_now();
        let dt = csharp_to_datetime(ticks);
        let now = Utc::now().naive_utc();
        let diff = (now - dt).num_seconds().abs();
        assert!(diff < 5, "Difference too high: {} seconds", diff);
    }
}
