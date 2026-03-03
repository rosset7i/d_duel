use std::hash::{DefaultHasher, Hash};

use rand_chacha::{
    ChaCha8Rng,
    rand_core::{Rng, SeedableRng},
};

#[derive(Clone)]
pub struct DeterministicRng {
    rng: ChaCha8Rng,
    seed: u64,
    calls: u8,
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

    pub fn hash(&self, hasher: &mut DefaultHasher) {
        self.seed.hash(hasher);
        self.calls.hash(hasher);
    }
}
