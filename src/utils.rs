use rand::{RngCore, SeedableRng};
use rand::rngs::SmallRng;

#[inline]
pub fn generate_random_number(seed: u64) -> u32 {
    let mut random = SmallRng::seed_from_u64(seed);
    random.next_u32()
}