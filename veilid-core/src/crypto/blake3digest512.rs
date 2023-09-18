use curve25519_dalek::digest::generic_array::{typenum::U64, GenericArray};
use curve25519_dalek::digest::{
    Digest, FixedOutput, FixedOutputReset, Output, OutputSizeUser, Reset, Update,
};

pub struct Blake3Digest512 {
    dig: blake3::Hasher,
}

impl OutputSizeUser for Blake3Digest512 {
    type OutputSize = U64;
}

impl Update for Blake3Digest512 {
    fn update(&mut self, data: &[u8]) {
        self.dig.update(data);
    }
}

impl FixedOutput for Blake3Digest512 {
    fn finalize_into(self, out: &mut Output<Self>) {
        let mut b = [0u8; 64];
        self.dig.finalize_xof().fill(&mut b);
        for n in 0..64 {
            out[n] = b[n];
        }
    }
}

impl Reset for Blake3Digest512 {
    fn reset(&mut self) {
        self.dig.reset();
    }
}

impl FixedOutputReset for Blake3Digest512 {
    fn finalize_into_reset(&mut self, out: &mut Output<Self>) {
        let mut b = [0u8; 64];
        self.dig.finalize_xof().fill(&mut b);
        for n in 0..64 {
            out[n] = b[n];
        }
        self.dig.reset();
    }
}

impl Digest for Blake3Digest512 {
    fn new() -> Self {
        Self {
            dig: blake3::Hasher::new(),
        }
    }

    fn new_with_prefix(data: impl AsRef<[u8]>) -> Self {
        Self::new().chain_update(data)
    }

    fn chain_update(mut self, data: impl AsRef<[u8]>) -> Self
    where
        Self: Sized,
    {
        <Self as Update>::update(&mut self, data.as_ref());
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
        self.dig.reset();
        out
    }

    fn output_size() -> usize {
        64
    }

    fn digest(data: impl AsRef<[u8]>) -> Output<Self> {
        let mut dig = blake3::Hasher::new();
        dig.update(data.as_ref());
        let mut b = [0u8; 64];
        dig.finalize_xof().fill(&mut b);
        let mut out = GenericArray::<u8, U64>::default();
        for n in 0..64 {
            out[n] = b[n];
        }
        out
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        <Self as Update>::update(self, data.as_ref())
    }

    fn finalize_into(self, out: &mut Output<Self>) {
        <Self as FixedOutput>::finalize_into(self, out)
    }

    fn finalize_into_reset(&mut self, out: &mut Output<Self>)
    where
        Self: FixedOutputReset,
    {
        <Self as FixedOutputReset>::finalize_into_reset(self, out)
    }

    fn reset(&mut self)
    where
        Self: Reset,
    {
        <Self as Reset>::reset(self);
    }
}
