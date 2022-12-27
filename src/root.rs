use crate::data::MerkleTreeData;
use crate::hash::MerkleTreeHash;
use crate::hash_fn::{MerkleTreeHashFn, DEFAULT_HASH_FN};
use crate::hash_pair::sort_hash_pair;
use crate::proof::MerkleTreeProof;
use near_sdk::borsh::BorshSerialize;

pub struct MerkleTreeRoot {
    hash: MerkleTreeHash,
    hash_fn: MerkleTreeHashFn,
}

impl MerkleTreeRoot {
    pub fn new(hash: &MerkleTreeHash, hash_fn: Option<MerkleTreeHashFn>) -> Self {
        MerkleTreeRoot {
            hash: hash.clone(),
            hash_fn: hash_fn.unwrap_or_else(|| DEFAULT_HASH_FN),
        }
    }

    pub fn verify(&self, data: &MerkleTreeData, proof: &MerkleTreeProof) -> bool {
        let mut hash = (self.hash_fn)(data);

        for second_hash in proof {
            hash = (self.hash_fn)(&sort_hash_pair(&hash, second_hash).try_to_vec().unwrap());
        }

        self.hash == hash
    }
}
