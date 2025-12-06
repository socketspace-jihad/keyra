
use std::hash::{Hash, Hasher};

#[inline(always)]
pub fn get_shard_id(key: &[u8], num_shards: usize)->usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish()) as usize % num_shards
}
