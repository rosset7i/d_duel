use std::hash::Hash;

use rand_chacha::{
    ChaCha8Rng,
    rand_core::{Rng, SeedableRng},
};

pub struct DeterministicRng {
    rng: ChaCha8Rng,
    seed: u64,
    calls: u64,
}

impl DeterministicRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            seed,
            calls: 0,
        }
    }

    pub fn roll(&mut self) -> u32 {
        self.calls += 1;
        self.rng.next_u32()
    }
}

impl Hash for DeterministicRng {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.calls.hash(state);
    }
}
