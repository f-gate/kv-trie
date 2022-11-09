extern crate digest;
use digest::{Digest, Output};

#[derive(Debug, PartialEq, Eq)]
pub enum TrieError {
    KeyDoesNotExist,
    ExpectedLeafGotBranch,
}

/// A node is either a branch or a leaf, leafs store data (aswell as other things). 
#[derive(Debug, PartialEq)]
pub enum NodeType<T> {
    Leaf(T),
    Branch,
}

// Helper Function
pub fn get_index_rep_of_hex(hash: &[u8]) -> Vec<u8> {
    hash
    .iter()
    .flat_map(|num| {
        // This will return the index related to the hex digit
        // Decimal | Hex | index
        // 255 = 0xff == 15,15,
        // 10 = 0x0A = 00,10,
        // 100 = 0x56 = 05,06, 
            [num / 16u8, num % 16u8]
        }).collect::<Vec<u8>>()
}

/// Hashes the item ready to be used as a key.
pub fn hash_me<T: Digest, K: Sized>(input: K) -> Output<T> {
// If something wierd is happening question this and go to
// https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
    unsafe {
        let slice = std::slice::from_raw_parts(
            (&input as *const K) as *const u8,
            std::mem::size_of::<K>()
        );
        let mut hasher = T::new();
        hasher.update(slice);
        hasher.finalize()
    }   
}
    