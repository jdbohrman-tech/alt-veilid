use super::*;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use js_sys::Date;

        pub fn get_timestamp() -> u64 {
            if is_browser() {
                return (Date::now() * 1000.0f64) as u64;
            } else {
                panic!("WASM requires browser environment");
            }
        }

        pub fn debug_ts(ts: u64) -> String {
            if is_browser() {
                let now = Date::new_0();
                now.set_time(Date::now());
                let date = Date::new_0();
                date.set_time((ts / 1000u64) as f64);

                let show_year = now.get_utc_full_year() != date.get_utc_full_year();
                let show_month = show_year || now.get_utc_month() != date.get_utc_month();
                let show_date = show_month || now.get_utc_date() != date.get_utc_date();

                format!("{}{}{}{}",
                    if show_year {
                        format!("{:04}/",date.get_utc_full_year())
                    } else {
                        "".to_owned()
                    },
                    if show_month {
                        format!("{:02}/",date.get_utc_month())
                    } else {
                        "".to_owned()
                    },
                    if show_date {
                        format!("{:02}-",date.get_utc_date())
                    } else {
                        "".to_owned()
                    },
                    format!("{:02}:{:02}:{:02}.{:04}",
                            date.get_utc_hours(),
                            date.get_utc_minutes(),
                            date.get_utc_seconds(),
                            date.get_utc_milliseconds()
                    ))
            } else {
                panic!("WASM requires browser environment");
            }
        }
    } else {
        use std::time::{SystemTime, UNIX_EPOCH};
        use chrono::{Datelike, Timelike};

        pub fn get_timestamp() -> u64 {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_micros() as u64,
                Err(_) => panic!("SystemTime before UNIX_EPOCH!"),
            }
        }

        pub fn debug_ts(ts: u64) -> String {
            let now = chrono::DateTime::<chrono::Utc>::from(SystemTime::now());
            let date = chrono::DateTime::<chrono::Utc>::from(UNIX_EPOCH + Duration::from_micros(ts));

            let show_year = now.year() != date.year();
            let show_month = show_year || now.month() != date.month();
            let show_date = show_month || now.day() != date.day();

            format!("{}{}{}{}",
                if show_year {
                    format!("{:04}/",date.year())
                } else {
                    "".to_owned()
                },
                if show_month {
                    format!("{:02}/",date.month())
                } else {
                    "".to_owned()
                },
                if show_date {
                    format!("{:02}-",date.day())
                } else {
                    "".to_owned()
                },
                format!("{:02}:{:02}:{:02}.{:04}",
                        date.hour(),
                        date.minute(),
                        date.second(),
                        date.nanosecond()/1_000_000
                ))

        }
    }
}

const DAY: u64 = 1_000_000u64 * 60 * 60 * 24;
const HOUR: u64 = 1_000_000u64 * 60 * 60;
const MIN: u64 = 1_000_000u64 * 60;
const SEC: u64 = 1_000_000u64;
const MSEC: u64 = 1_000u64;

pub fn debug_duration(dur: u64) -> String {
    let days = dur / DAY;
    let dur = dur % DAY;
    let hours = dur / HOUR;
    let dur = dur % HOUR;
    let mins = dur / MIN;
    let dur = dur % MIN;
    let secs = dur / SEC;
    let dur = dur % SEC;
    let msecs = dur / MSEC;

    format!(
        "{}{}{}{}.{:03}s",
        if days != 0 {
            format!("{}d", days)
        } else {
            "".to_owned()
        },
        if hours != 0 {
            format!("{}h", hours)
        } else {
            "".to_owned()
        },
        if mins != 0 {
            format!("{}m", mins)
        } else {
            "".to_owned()
        },
        secs,
        msecs
    )
}
