use data::MerkleTreeData;
use hash::MerkleTreeHash;
use hash_fn::{MerkleTreeHashFn, DEFAULT_HASH_FN};
use hash_pair::sort_hash_pair;
use near_sdk::borsh::BorshSerialize;
use proof::MerkleTreeProof;
use root::MerkleTreeRoot;

pub mod data;
pub mod hash;
pub mod hash_fn;
pub mod hash_pair;
pub mod proof;
pub mod root;

pub struct MerkleTree {
    root: MerkleTreeRoot,
    nodes: Vec<MerkleTreeHash>,
    st_sum: usize,
}

impl MerkleTree {
    pub fn create(items: &Vec<MerkleTreeData>, hash_fn: Option<MerkleTreeHashFn>) -> Self {
        let mut items = items.clone();

        let hash_fn = hash_fn.unwrap_or_else(|| DEFAULT_HASH_FN);

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
            nodes[i] = hash_fn(&items[i - st_sum]);
        }

        let mut i = st_sum.clone();

        while i > 0 {
            i -= 1;

            nodes[i] = hash_fn(
                &sort_hash_pair(&nodes[(i << 1) + 1], &nodes[(i + 1) << 1])
                    .try_to_vec()
                    .unwrap(),
            );
        }

        MerkleTree {
            root: MerkleTreeRoot::new(&nodes[0], Some(hash_fn)),
            nodes,
            st_sum,
        }
    }

    pub fn get_proof(&self, index: usize) -> MerkleTreeProof {
        let mut result = MerkleTreeProof::new();

        let mut v = index + self.st_sum;

        while v > 0 {
            let w = if v % 2 == 0 { v - 1 } else { v + 1 };

            result.push(self.nodes[w]);

            v = (v - 1) >> 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn correct_number_of_nodes() {
        let mut items = Vec::<MerkleTreeData>::new();

        for i in 0..8 {
            items.push(i.try_to_vec().unwrap());
        }

        let merkle_tree = MerkleTree::create(&items, None);

        assert_eq!(merkle_tree.nodes.len(), 15);
    }

    #[test]
    pub fn verify_correct_data() {
        let mut items = Vec::<MerkleTreeData>::new();

        for i in 0..4 {
            items.push(i.try_to_vec().unwrap());
        }

        let merkle_tree = MerkleTree::create(&items, None);

        for i in 0..items.len() {
            assert!(merkle_tree
                .root
                .verify(&items[i], &merkle_tree.get_proof(i)));
        }
    }

    #[test]
    pub fn verify_incorrect_data() {
        let mut items = Vec::<MerkleTreeData>::new();

        for i in 0..4 {
            items.push(i.try_to_vec().unwrap());
        }

        let merkle_tree = MerkleTree::create(&items, None);

        assert!(!merkle_tree
            .root
            .verify(&items[0], &merkle_tree.get_proof(1)));
    }

    // #[test]
    // pub fn print_tree() {
    //     let mut items = Vec::<MerkleTreeData>::new();

    //     for i in 0..4 {
    //         items.push(i.try_to_vec().unwrap());
    //     }

    //     let merkle_tree = MerkleTree::create(&items, None);

    //     let mut st = 1_usize;
    //     let mut count = 0_usize;

    //     for node in merkle_tree.nodes {
    //         print!("{:?}", node);

    //         count += 1;
    //         if count == st {
    //             println!("{}", "-".repeat(30));
    //             st <<= 1;
    //             count = 0;
    //         }
    //     }
    // }
}
