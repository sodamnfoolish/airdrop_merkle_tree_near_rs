use crate::data::MerkleTreeData;
use crate::hash::MerkleTreeHash;
use near_sdk::env::keccak256_array;

pub type MerkleTreeHashFn = fn(&MerkleTreeData) -> MerkleTreeHash;

pub const DEFAULT_HASH_FN: MerkleTreeHashFn = |data| keccak256_array(data);
