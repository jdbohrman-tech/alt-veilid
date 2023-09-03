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
        random_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        random_bytes(dest);
        Ok(())
    }
}

pub fn random_bytes(dest: &mut [u8]) {
    let mut rng = rand::thread_rng();
    rng.fill_bytes(dest);
}

pub fn get_random_u32() -> u32 {
    let mut rng = rand::thread_rng();
    rng.next_u32()
}

pub fn get_random_u64() -> u64 {
    let mut rng = rand::thread_rng();
    rng.next_u64()
}
