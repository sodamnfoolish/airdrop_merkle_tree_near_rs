use crate::data::MerkleTreeData;
use crate::hash_pair::sort_hash_pair;
use crate::proof::MerkleTreeProof;
use crate::root::MerkleTreeRoot;
use near_sdk::borsh::BorshSerialize;
use near_sdk::env::keccak256_array;

pub mod data;
pub mod hash;
pub mod hash_pair;
pub mod proof;
pub mod root;

pub struct MerkleTree {
    pub root: MerkleTreeRoot,
    pub proofs: Vec<MerkleTreeProof>,
}

impl MerkleTree {
    pub fn build(items: &Vec<MerkleTreeData>) -> Self {
        let items_len = items.len();

        let mut items = items.clone();

        let mut st_sum = 0_usize;
        let mut st = 1_usize;

        while st < items.len() {
            st_sum += st;

            st <<= 1;
        }

        while items.len() < st_sum + st {
            items.push(MerkleTreeData::new());
        }

        let mut nodes = vec![[0_u8; 32]; st_sum + st];

        for i in st_sum..st_sum + st {
            nodes[i] = keccak256_array(&items[i - st_sum]);
        }

        let mut i = st_sum.clone();

        while i > 0 {
            i -= 1;

            nodes[i] = keccak256_array(
                &sort_hash_pair(&nodes[(i << 1) + 1], &nodes[(i + 1) << 1])
                    .try_to_vec()
                    .unwrap(),
            );
        }

        let get_proof = |index: usize| -> MerkleTreeProof {
            let mut result = MerkleTreeProof::new();

            let mut v = index + st_sum;

            while v > 0 {
                let w = if v % 2 == 0 { v - 1 } else { v + 1 };

                result.push(nodes[w]);

                v = (v - 1) >> 1;
            }

            result
        };

        let mut proofs: Vec<MerkleTreeProof> = Vec::new();

        for i in 0..items_len {
            proofs.push(get_proof(i))
        }

        MerkleTree {
            root: MerkleTreeRoot::new(nodes[0]),
            proofs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn correct_proofs() {
        let mut items = Vec::<MerkleTreeData>::new();

        for i in 0..8 {
            items.push(i.try_to_vec().unwrap());
        }

        let merkle_tree = MerkleTree::build(&items);

        assert_eq!(merkle_tree.proofs.len(), 8);

        for proof in merkle_tree.proofs {
            assert_eq!(proof.len(), 3);
        }
    }

    #[test]
    pub fn verify_correct_data() {
        let mut items = Vec::<MerkleTreeData>::new();

        for i in 0..4 {
            items.push(i.try_to_vec().unwrap());
        }

        let merkle_tree = MerkleTree::build(&items);

        for i in 0..items.len() {
            assert!(merkle_tree.root.verify(&items[i], &merkle_tree.proofs[i]));
        }
    }

    #[test]
    pub fn verify_incorrect_data() {
        let mut items = Vec::<MerkleTreeData>::new();

        for i in 0..4 {
            items.push(i.try_to_vec().unwrap());
        }

        let merkle_tree = MerkleTree::build(&items);

        assert!(!merkle_tree.root.verify(&items[0], &merkle_tree.proofs[1]));
    }
}
