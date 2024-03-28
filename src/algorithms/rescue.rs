use crate::{prelude::*, Hasher};
use bytemuck::cast_slice;
use rescue_prime_optimized::RescuePrimeOptimized;

/// [`Hasher`]: crate::Hasher
#[derive(Clone)]
// pub struct Sha256Algorithm {}
pub struct RescueAlgorithm {}

impl Hasher for RescueAlgorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut padded_data = data.to_vec();

        while padded_data.len() % 8 != 0 {
            padded_data.push(0);
        }
        let u64_slice: &[u64] = cast_slice(&padded_data);
        let hash_result = RescuePrimeOptimized(u64_slice.to_vec());
        let hash_u8_slice: &[u8] = cast_slice(&hash_result);
        assert!(
            hash_u8_slice.len() == 32,
            "rescue hash result length invalid"
        );
        return core::convert::TryInto::try_into(hash_u8_slice).unwrap();
    }
}
