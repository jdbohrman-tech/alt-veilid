use super::*;
use rand::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct VeilidRng;

impl CryptoRng for VeilidRng {}

impl RngCore for VeilidRng {
    fn next_u32(&mut self) -> u32 {
        get_random_u32()
    }

    fn next_u64(&mut self) -> u64 {
        get_random_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        if let Err(e) = self.try_fill_bytes(dest) {
            panic!("Error: {}", e);
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        random_bytes(dest).map_err(rand::Error::new)
    }
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub fn random_bytes(dest: &mut [u8]) -> EyreResult<()> {
            let len = dest.len();
            let u32len = len / 4;
            let remlen = len % 4;

            for n in 0..u32len {
                let r = (Math::random() * (u32::max_value() as f64)) as u32;

                dest[n * 4 + 0] = (r & 0xFF) as u8;
                dest[n * 4 + 1] = ((r >> 8) & 0xFF) as u8;
                dest[n * 4 + 2] = ((r >> 16) & 0xFF) as u8;
                dest[n * 4 + 3] = ((r >> 24) & 0xFF) as u8;
            }
            if remlen > 0 {
                let r = (Math::random() * (u32::max_value() as f64)) as u32;
                for n in 0..remlen {
                    dest[u32len * 4 + n] = ((r >> (n * 8)) & 0xFF) as u8;
                }
            }

            Ok(())
        }

        pub fn get_random_u32() -> u32 {
            (Math::random() * (u32::max_value() as f64)) as u32
        }

        pub fn get_random_u64() -> u64 {
            let v1: u32 = get_random_u32();
            let v2: u32 = get_random_u32();
            ((v1 as u64) << 32) | ((v2 as u32) as u64)
        }

    } else {

        pub fn random_bytes(dest: &mut [u8]) -> EyreResult<()> {
            let mut rng = rand::thread_rng();
            rng.try_fill_bytes(dest).wrap_err("failed to fill bytes")
        }

        pub fn get_random_u32() -> u32 {
            let mut rng = rand::thread_rng();
            rng.next_u32()
        }

        pub fn get_random_u64() -> u64 {
            let mut rng = rand::thread_rng();
            rng.next_u64()
        }
    }
}
