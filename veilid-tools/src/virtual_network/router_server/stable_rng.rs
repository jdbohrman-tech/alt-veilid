use super::*;

use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

#[derive(Clone)]
pub struct StableRngState {
    srng: ChaCha20Rng,
    count: usize,
}

impl fmt::Debug for StableRngState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StableRngInner")
            .field("count", &self.count)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct StableRng {
    state: StableRngState,
}

impl StableRng {
    ////////////////////////////////////////////////////////
    // Public Interface

    pub fn new(seed: u64) -> Self {
        Self {
            state: StableRngState {
                srng: ChaCha20Rng::seed_from_u64(seed),
                count: 0,
            },
        }
    }

    pub fn save_state(&self) -> StableRngState {
        self.state.clone()
    }

    pub fn restore_state(&mut self, state: StableRngState) {
        self.state = state;
    }

    pub fn probability_test(&mut self, probability: Probability) -> bool {
        if probability == 1.0 {
            return true;
        } else if probability == 0.0 {
            return false;
        }
        let num = self.next_f32(0.0, 1.0);
        num < probability
    }

    pub fn weighted_choice_ref<'a, T: fmt::Debug + Clone>(
        &mut self,
        weighted_list: &'a WeightedList<T>,
    ) -> &'a T {
        match weighted_list {
            WeightedList::Single(x) => x,
            WeightedList::List(vec) => {
                let total_weight = vec
                    .iter()
                    .map(|x| x.weight())
                    .reduce(|acc, x| acc + x)
                    .expect("config validation broken");

                let r = self.next_f32(0.0, total_weight);
                let mut current_weight = 0.0f32;
                for x in vec {
                    current_weight += x.weight();
                    if r < current_weight {
                        return x.item();
                    }
                }
                // Catch f32 imprecision
                vec.last().expect("config validation broken").item()
            }
        }
    }

    pub fn weighted_choice<T: fmt::Debug + Clone>(&mut self, weighted_list: WeightedList<T>) -> T {
        match weighted_list {
            WeightedList::Single(x) => x,
            WeightedList::List(mut vec) => {
                let total_weight = vec
                    .iter()
                    .map(|x| x.weight())
                    .reduce(|acc, x| acc + x)
                    .expect("config validation broken");

                let r = self.next_f32(0.0, total_weight);
                let mut current_weight = 0.0f32;
                let last = vec.pop().expect("config validation broken").into_item();
                for x in vec {
                    current_weight += x.weight();
                    if r < current_weight {
                        return x.into_item();
                    }
                }
                // Catch f32 imprecision
                last
            }
        }
    }

    pub fn shuffle_vec<T>(&mut self, v: &mut Vec<T>) {
        self.state.count += 1;
        v.shuffle(&mut self.state.srng);
    }

    pub fn next_u32(&mut self, min: u32, max: u32) -> u32 {
        self.state.count += 1;
        self.state.srng.gen_range(min..=max)
    }

    pub fn next_u128(&mut self, min: u128, max: u128) -> u128 {
        self.state.count += 1;
        self.state.srng.gen_range(min..=max)
    }

    pub fn next_f32(&mut self, min: f32, max: f32) -> f32 {
        self.state.count += 1;
        self.state.srng.gen_range(min..=max)
    }
}
