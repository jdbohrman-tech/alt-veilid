use curve25519_dalek::digest::generic_array::typenum::U64;
use curve25519_dalek::digest::{Digest, Output};
use generic_array::GenericArray;

pub struct Blake3Digest512 {
    dig: blake3::Hasher,
}

impl Digest for Blake3Digest512 {
    type OutputSize = U64;

    fn new() -> Self {
        Self {
            dig: blake3::Hasher::new(),
        }
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.dig.update(data.as_ref());
    }

    fn chain(mut self, data: impl AsRef<[u8]>) -> Self
    where
        Self: Sized,
    {
        self.update(data);
        self
    }

    fn finalize(self) -> Output<Self> {
        let mut b = [0u8; 64];
        self.dig.finalize_xof().fill(&mut b);
        let mut out = GenericArray::<u8, U64>::default();
        for n in 0..64 {
            out[n] = b[n];
        }
        out
    }

    fn finalize_reset(&mut self) -> Output<Self> {
        let mut b = [0u8; 64];
        self.dig.finalize_xof().fill(&mut b);
        let mut out = GenericArray::<u8, U64>::default();
        for n in 0..64 {
            out[n] = b[n];
        }
        self.reset();
        out
    }

    fn reset(&mut self) {
        self.dig.reset();
    }

    fn output_size() -> usize {
        64
    }

    fn digest(data: &[u8]) -> Output<Self> {
        let mut dig = blake3::Hasher::new();
        dig.update(data);
        let mut b = [0u8; 64];
        dig.finalize_xof().fill(&mut b);
        let mut out = GenericArray::<u8, U64>::default();
        for n in 0..64 {
            out[n] = b[n];
        }
        out
    }
}
