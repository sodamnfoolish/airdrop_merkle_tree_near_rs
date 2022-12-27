use crate::data::MerkleTreeData;
use crate::hash::MerkleTreeHash;
use crate::hash_fn::{MerkleTreeHashFn, DEFAULT_HASH_FN};
use crate::hash_pair::sort_hash_pair;
use crate::proof::MerkleTreeProof;
use near_sdk::borsh::BorshSerialize;

pub fn verify(
    root_hash: &MerkleTreeHash,
    data: &MerkleTreeData,
    proof: &MerkleTreeProof,
    hash_fn: Option<MerkleTreeHashFn>,
) -> bool {
    let hash_fn = hash_fn.unwrap_or_else(|| DEFAULT_HASH_FN);

    let mut hash = (hash_fn)(data);

    for second_hash in proof {
        hash = (hash_fn)(&sort_hash_pair(&hash, second_hash).try_to_vec().unwrap());
    }

    root_hash == &hash
}
