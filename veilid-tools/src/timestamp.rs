use super::*;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use js_sys::Date;

        pub fn get_timestamp() -> u64 {
            if utils::is_browser() {
                return (Date::now() * 1000.0f64) as u64;
            } else {
                panic!("WASM requires browser environment");
            }
        }
    } else {
        use std::time::{SystemTime, UNIX_EPOCH};

        pub fn get_timestamp() -> u64 {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_micros() as u64,
                Err(_) => panic!("SystemTime before UNIX_EPOCH!"),
            }
        }

    }
}
