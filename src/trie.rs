
use digest::*;
use blake2::*;
use core::marker::PhantomData;

<<<<<<< HEAD
#[derive(Debug, PartialEq, Eq)]
pub enum TrieError {
    KeyDoesNotExist,
    ExpectedLeafGotBranch,
}

// Todo, we need this to be as small as possible.
// Store data elsewhere? like a hashmap?
// Or remove the need for option somehow.
pub struct ChildNode<I: Sized + Clone> {
    // Boxed due to recursive type.
    nodes: Box<[Option<ChildNode<I>>; 16]>,
    node_type: NodeType<I>,
}

impl<I: Sized + Clone> ChildNode<I> {
    // Create a empty which is of type branch.
    fn new(maybe_type: Option<I>) -> ChildNode<I> {
        let mut node_type = NodeType::Branch;
        if maybe_type.is_some() {
            node_type = NodeType::Leaf(maybe_type.expect("is_some() is called above; qed"));
        }
        Self {
            nodes: Box::new([None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]),
            node_type
        }
    }

    fn insert(&mut self, key: &[u8], data: &I) -> Result<(), ()> {
        // Check to see if the node exists in the children.
        // If it does recurse deeper.
        let key_len = key.len(); 
        if self.nodes[key[0] as usize].is_some() {
            if key_len > 1 {
                self.nodes[key[0] as usize]
                .as_mut()
                .expect("node has been found; qed")
                .insert(&key[1..], data)?;
            } else {
                // Leaf node exists, edit.
                self.nodes[key[0] as usize].as_mut().expect("node has been found; qed").node_type = NodeType::<I>::Leaf(data.clone());
            }
        } else {
        // Alas if we have not found a node then we must create our new recursive node and keep the existing data.
            if key_len > 1 {
                // Here we instantiate the new optional nodes and set the key to the new node.
                let mut new_node = ChildNode::new( 
                    None
                );
                // Again we must continue recursion on the new node until key is len 1.
                    new_node.insert(&key[1..], data)?;
                // This is done after recursion because we need that node to be populated with the nested nodes within it.
                self.nodes[key[0] as usize] = Some(new_node);
            } else {
                // Leaf node does not exist, create.
                let new_node = ChildNode::new( 
                    Some(data.clone())
                );
                self.nodes[key[0] as usize] = Some(new_node);
            }
        }
        Ok(())
    }

    // Recurse up to the leaf node and return the element.
    fn get(&self, key: &[u8]) -> Result<I, TrieError> {
        if let Some(node) = &self.nodes[key[0] as usize] {
            if key.len() > 1 {
                node.get(&key[1..])
            } else {
                match &node.node_type {
                    NodeType::Leaf(data) => Ok(data.clone()),
                    _ => Err(TrieError::ExpectedLeafGotBranch)
                }
            }
        } else {
            Err(TrieError::KeyDoesNotExist)
        }

    }

    fn remove(&mut self, key: &[u8]) -> Result<(), TrieError> {
        // Go and find the leaf node
        // If found then recurse up the tree deleting either:
        // 1: The entire node if there is no other nodes associated and continue recursion.
        // 2: Just the associated node if there is other nodes and return Ok(())  

        Ok(())
    }
}

#[derive(Debug)]
pub enum NodeType<T> {
    Leaf(T),
    Branch,
}

=======
use crate::childnode::ChildNode;
use crate::utils::*;
>>>>>>> 5ac2eafcb0aa374698245d3057a3a27de4d0fbc0
/// The trie, currently programmed as base 16.
/// Uses the hash on the key inputted to compute a place in storage.
pub struct Trie<T: Digest, K: Sized, I: Sized + Clone> {
    pub root: ChildNode<I>,
    phantom_t: PhantomData<T>,
    phantom_k: PhantomData<K>,
}

impl<T, K, I> Trie<T, K, I>
where
    //The hasher used on the key.
    T: Digest,
    //The key.
    K: Sized,
    //The data you intend to store.
    I: Sized + Clone,
 {
    pub fn new() -> Self {
        Self {
            root: ChildNode::new_branch(0),
            phantom_k: PhantomData,
            phantom_t: PhantomData,
        }
    }

    /// Insert an item into the trie.
    pub fn insert(&mut self, key: K, data: I) -> Result<(), ()> {
        let hash_bytes = hash_me::<T, K>(key);
        // Compute the "decimal index representation of hex", a necessary evil for the behaivour of the hex trie .
        let index_representation = get_index_rep_of_hex(hash_bytes.as_slice());

        self.root.insert(
            index_representation.as_slice(),
            &data,
        )  
    }

    /// Get an item from the trie.
    pub fn get(&self, key: K) -> Result<I, TrieError> {
        let hash_bytes = hash_me::<T, K>(key);

        let index_of_hex = get_index_rep_of_hex(hash_bytes.as_slice());
        self.root.get(index_of_hex.as_slice())
    }

    /// Remove an item from the trie.
    pub fn remove(&mut self, key: K) -> Result<bool, TrieError> {
        let hash_bytes = hash_me::<T, K>(key);

        let index_of_hex = get_index_rep_of_hex(hash_bytes.as_slice());
        self.root.remove(index_of_hex.as_slice(), index_of_hex.len())
    }

    fn write_to_disk() {
        todo!()
    }

    // An idea to add patricia trie optimisation.
    fn _optimise_patricia(){
        todo!()
    }
}


<<<<<<< HEAD
// for numbers below 255 only
fn decimal_to_hex_index(decimal: u8) -> [u8; 2] {
    [decimal / 16u8, decimal % 16u8]
}

fn hash_me<T: Digest, K: Sized>(input: K) -> Output<T> {
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
=======
>>>>>>> 5ac2eafcb0aa374698245d3057a3a27de4d0fbc0

#[test]
fn test_insert_and_retrieve_spam() {
    let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();

    let max: u64 = 10_000;
    let input: Vec<String> = (0..max).into_iter().map(|i| {
        i.to_string()
    }).collect();
    let _ = (0..max).into_iter().map(|i| {
        let _ = trie.insert(input[i as usize].as_str(), i);
    }).collect::<()>();
    
    let _ = (0..max).into_iter().map(|i| {
        let res = trie.get(input[i as usize].as_str());
        assert_eq!(res, Ok(i));
    }).collect::<()>();
}

#[test]
fn test_retrive_nothing_errs() {
    let mut trie: Trie<Blake2b512, &str, f32> = Trie::new();
    assert!(trie.insert("hello", 60f32).is_ok());
    assert_eq!(trie.get("hello_world !!12345"), Err(TrieError::KeyDoesNotExist));
}

<<<<<<< HEAD
=======
#[test]
fn test_insert_and_remove() {
    let mut trie: Trie<Blake2b512, f32, u64> = Trie::new();
    assert!(trie.insert(1000f32, 60u64).is_ok());
    assert_eq!(trie.get(1000f32).unwrap(), 60u64);
    assert!(trie.remove(1000f32).is_ok());
    assert!(trie.get(1000f32).is_err());
}


#[test]
fn test_remove_crazy() {
    let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
    let num = 10_000u64;
    let input: Vec<String> = (0..num).into_iter().map(|i| {i.to_string()}).collect();

    let _ =  (0..num).into_iter().map(|i| {

        let _ = trie.insert(input[i as usize].as_str(), num);
        let res = trie.remove(input[i as usize].as_str()); 
        assert!(res.is_ok());

        let res = trie.get(input[i as usize].as_str()); 
        assert!(res.is_err());
    }).collect::<()>();
>>>>>>> 5ac2eafcb0aa374698245d3057a3a27de4d0fbc0

