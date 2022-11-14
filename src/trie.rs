
use digest::*;
use blake2::*;
use core::marker::PhantomData;

use crate::childnode::ChildNode;
use crate::utils::*;

pub trait StorageConfig {
    type Key: Sized;
    type Hasher: Digest;
    type Data: Sized + Clone;
}

pub trait StorageMethod<T: StorageConfig> {
    fn get(&self, key: T::Key) -> Result<T::Data, TrieError>;
    fn insert(&mut self, key: T::Key, data: T::Data) -> Result<(), ()>;
    fn remove(&mut self, key: T::Key) -> Result<bool, TrieError>;
    fn new() -> Self;
}

/// The trie, currently programmed as base 16.
/// Uses the hash on the key inputted to compute a place in storage.
#[derive(Debug)]
pub struct Trie<T: StorageConfig> {
    pub root: ChildNode<T::Data>,
}

impl<T: StorageConfig> StorageMethod<T> for Trie<T>{
    fn new() -> Self {
        Self {
            root: ChildNode::new_branch(0),
        }
    }

    /// Insert an item into the trie.
    fn insert(&mut self, key: T::Key, data: T::Data) -> Result<(), ()> {
        let hash_bytes = hash_me::<T::Hasher, T::Key>(key);
        // Compute the "decimal index representation of hex", a necessary evil for the behaivour of the hex trie .
        let index_representation = get_index_rep_of_hex(hash_bytes.as_slice());

        self.root.insert(
            index_representation.as_slice(),
            &data,
        )  
    }

    /// Get an item from the trie.
    fn get(&self, key: T::Key) -> Result<T::Data, TrieError> {
        let hash_bytes = hash_me::<T::Hasher, T::Key>(key);

        let index_of_hex = get_index_rep_of_hex(hash_bytes.as_slice());
        self.root.get(index_of_hex.as_slice())
    }

    /// Remove an item from the trie.
    fn remove(&mut self, key: T::Key) -> Result<bool, TrieError> {
        let hash_bytes = hash_me::<T::Hasher, T::Key>(key);

        let index_of_hex = get_index_rep_of_hex(hash_bytes.as_slice());
        self.root.remove(index_of_hex.as_slice(), index_of_hex.len())
    }
}



// #[test]
// fn test_insert_and_retrieve_spam() {
//     let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();

//     let max: u64 = 10_000;
//     let input: Vec<String> = (0..max).into_iter().map(|i| {
//         i.to_string()
//     }).collect();
//     let _ = (0..max).into_iter().map(|i| {
//         let _ = trie.insert(input[i as usize].as_str(), i);
//     }).collect::<()>();
    
//     let _ = (0..max).into_iter().map(|i| {
//         let res = trie.get(input[i as usize].as_str());
//         assert_eq!(res, Ok(i));
//     }).collect::<()>();
// }

// #[test]
// fn test_retrive_nothing_errs() {
//     let mut trie: Trie<Blake2b512, &str, f32> = Trie::new();
//     assert!(trie.insert("hello", 60f32).is_ok());
//     assert_eq!(trie.get("hello_world !!12345"), Err(TrieError::KeyDoesNotExist));
// }

// #[test]
// fn test_insert_and_remove() {
//     let mut trie: Trie<Blake2b512, f32, u64> = Trie::new();
//     assert!(trie.insert(1000f32, 60u64).is_ok());
//     assert_eq!(trie.get(1000f32).unwrap(), 60u64);
//     assert!(trie.remove(1000f32).is_ok());
//     assert!(trie.get(1000f32).is_err());
// }


// #[test]
// fn test_remove_crazy() {
//     let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
//     let num = 10_000u64;
//     let input: Vec<String> = (0..num).into_iter().map(|i| {i.to_string()}).collect();

//     let _ =  (0..num).into_iter().map(|i| {

//         let _ = trie.insert(input[i as usize].as_str(), num);
//         let res = trie.remove(input[i as usize].as_str()); 
//         assert!(res.is_ok());

//         let res = trie.get(input[i as usize].as_str()); 
//         assert!(res.is_err());
//     }).collect::<()>();
// }


// #[test]
// fn test_multithread() {
//     let _trie: Trie<Blake2b512, &str, u64> = Trie::new();
// }

