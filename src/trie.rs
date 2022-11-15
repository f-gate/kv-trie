
use std::collections::HashMap;
use std::marker::PhantomData;
use digest::*;
use blake2::*;
use crate::childnode::ChildNode;
use crate::utils::*;

pub trait StorageMethod<T: Sized + Clone, K: Sized, I: Digest> {

    fn get(&self, key: K) -> Result<T, TrieError>;
    fn insert(&mut self, key: K, data: T) -> Result<(), ()>;
    fn remove(&mut self, key: K) -> Result<bool, TrieError>;
    fn new() -> Self;
}

/// The trie, currently programmed as base 16.
/// Uses the hash on the key inputted to compute a place in storage.
#[derive(Debug)]
pub struct Trie<T: Sized + Clone, K: Sized + Clone, I: Digest> {
    pub root: ChildNode<T>,
    pub avaliable_keys: HashMap<K, ()>,
    phantom_k: PhantomData<K>,
    phantom_t: PhantomData<I>,

}

impl <T: Sized + Clone, K: Sized + Clone, I: Digest> StorageMethod<T, K, I> for Trie<T, K, I>{

    fn new() -> Self {
        Self {
            root: ChildNode::new_branch(0),
            phantom_k: PhantomData,
            phantom_t: PhantomData,
            avaliable_keys: HashMap::new(),
        }
    }

    /// Insert an item into the trie.
    fn insert(&mut self, key: K, data: T) -> Result<(), ()> {
        let hash_bytes = hash_me::<I, K>(key);
        // Compute the "decimal index representation of hex", a necessary evil for the behaivour of the hex trie .
        let index_representation = get_index_rep_of_hex(hash_bytes.as_slice());

        let _ = self.root.insert(
            index_representation.as_slice(),
            &data,
        )?;
            self.avaliable_keys.insert(key.clone(), ());
        Ok(())
    }

    /// Get an item from the trie.
    fn get(&self, key: K) -> Result<T, TrieError> {
        let hash_bytes = hash_me::<I, K>(key);

        let index_of_hex = get_index_rep_of_hex(hash_bytes.as_slice());
        self.root.get(index_of_hex.as_slice())
    }

    /// Remove an item from the trie.
    fn remove(&mut self, key: K) -> Result<bool, TrieError> {
        let hash_bytes = hash_me::<I, K>(key);

        let index_of_hex = get_index_rep_of_hex(hash_bytes.as_slice());
        let _ = self.root.remove(index_of_hex.as_slice(), index_of_hex.len())?;
            
        self.avaliable_keys.remove(key)
    }
}



 #[test]
 fn test_insert_and_retrieve_spam() {
     let mut trie: Trie<u64, &str, Blake2b512> = Trie::new();
     let max: u64 = 10_000;
     let input: Vec<String> = (0..max).into_iter().map(|i| {
         i.to_string()
     }).collect();
     let _: Vec<_> = (0..max).into_iter().map(|i| {
         let _ = trie.insert(input[i as usize].as_str(), i);
     }).collect();  
     let _: Vec<_> = (0..max).into_iter().map(|i| {
         let res = trie.get(input[i as usize].as_str());
         assert_eq!(res, Ok(i));
     }).collect();
}
 #[test]
 fn test_retrive_nothing_errs() {
     let mut trie: Trie<f32, &str, Blake2b512> = Trie::new();
     assert!(trie.insert("hello", 60f32).is_ok());
     assert_eq!(trie.get("hello_world !!12345"), Err(TrieError::KeyDoesNotExist));
 } 
 #[test]
 fn test_insert_and_remove() {
     let mut trie: Trie<u64, f32,  Blake2b512> = Trie::new();
     assert!(trie.insert(1000f32, 60u64).is_ok());
     assert_eq!(trie.get(1000f32).unwrap(), 60u64);
     assert!(trie.remove(1000f32).is_ok());
     assert!(trie.get(1000f32).is_err());
 
 }
#[test]
 fn test_remove_crazy() {
     let mut trie: Trie<u64, &str, Blake2b512> = Trie::new();
     let num = 10_000u64;


     let input: Vec<String> = (0..num).into_iter().map(|i| {i.to_string()}).collect();
     let _: Vec<_> =  (0..num).into_iter().map(|i| { 
         let _ = trie.insert(input[i as usize].as_str(), num);

         let res = trie.remove(input[i as usize].as_str()); 
         assert!(res.is_ok());
         let res = trie.get(input[i as usize].as_str()); 
         assert!(res.is_err());
     }).collect();
}