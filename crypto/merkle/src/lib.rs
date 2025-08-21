use bascoin_hashes::{Hash, Hasher, HasherBase, MerkleBranchHash, ZERO_HASH};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

pub fn calc_merkle_root(hashes: impl ExactSizeIterator<Item = Hash>) -> Hash {
    if hashes.len() == 0 {
        return ZERO_HASH;
    }
    let next_pot = hashes.len().next_power_of_two();
    let vec_len = 2 * next_pot - 1;
    let mut merkles = vec![None; vec_len];
    for (i, hash) in hashes.enumerate() {
        merkles[i] = Some(hash);
    }
    let mut offset = next_pot;
    for i in (0..vec_len - 1).step_by(2) {
        if merkles[i].is_none() {
            merkles[offset] = None;
        } else {
            merkles[offset] = Some(merkle_hash(merkles[i].unwrap(), merkles[i + 1].unwrap_or(ZERO_HASH)));
        }
        offset += 1
    }
    merkles.last().unwrap().unwrap()
}

pub fn merkle_hash(left: Hash, right: Hash) -> Hash {
    let mut hasher = MerkleBranchHash::new();
    hasher.update(left).update(right);
    hasher.finalize()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct MerkleProof {
    pub hashes: Vec<Hash>,
    pub flags: Vec<bool>,
}

impl MerkleProof {
    pub fn new(hashes: Vec<Hash>, flags: Vec<bool>) -> Self {
        Self { hashes, flags }
    }

    pub fn apply(&self, mut hash: Hash) -> Hash {
        for (hash_to_combine, flag) in self.hashes.iter().zip(self.flags.iter()) {
            hash = if *flag {
                // The hash to combine is on the right
                MerkleBranchHash::hash(&[hash.as_bytes(), hash_to_combine.as_bytes()].concat())
            } else {
                // The hash to combine is on the left
                MerkleBranchHash::hash(&[hash_to_combine.as_bytes(), hash.as_bytes()].concat())
            };
        }
        hash
    }
}
