use crate::hash::MerkleTreeHash;

pub fn sort_hash_pair(
    first: &MerkleTreeHash,
    second: &MerkleTreeHash,
) -> (MerkleTreeHash, MerkleTreeHash) {
    if first < second {
        (first.clone(), second.clone())
    } else {
        (second.clone(), first.clone())
    }
}
