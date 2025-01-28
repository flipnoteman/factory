use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

#[inline]
pub fn generate_random_number(seed: u64) -> u32 {
    let mut random = SmallRng::seed_from_u64(seed);
    random.next_u32()
}
