use crate::data::MerkleTreeData;
use crate::hash::MerkleTreeHash;
use crate::hash_pair::sort_hash_pair;
use crate::proof::MerkleTreeProof;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::keccak256_array;

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MerkleTreeRoot {
    pub hash: MerkleTreeHash,
}

impl MerkleTreeRoot {
    pub fn new(hash: MerkleTreeHash) -> Self {
        MerkleTreeRoot { hash }
    }

    pub fn verify(&self, data: &MerkleTreeData, proof: &MerkleTreeProof) -> bool {
        let mut hash = keccak256_array(data);

        for second_hash in proof {
            hash = keccak256_array(&sort_hash_pair(&hash, second_hash).try_to_vec().unwrap());
        }

        self.hash == hash
    }
}
