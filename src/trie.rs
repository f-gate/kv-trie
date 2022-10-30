use digest::*;
use blake2::*;
use core::marker::PhantomData;
use std::ops::Deref;


#[derive(Debug, PartialEq, Eq)]
pub enum TrieError {
    KeyDoesNotExist,
    ExpectedLeafGotBranch,
}

#[derive(Debug)]
pub struct ChildNode<I: Sized + Clone> {
    // Boxed due to recursive type.
    index_value: u8,
    pub nodes: Box<Vec<ChildNode<I>>>,
    pub node_type: NodeType<I>,
}

impl<I: Sized + Clone> ChildNode<I> {

    fn new_branch(index_value: u8) -> ChildNode<I> {
        Self {
            nodes: Box::new(vec![]),
            node_type: NodeType::Branch,
            index_value,
        }
    }
    fn new_leaf(index_value: u8, data: I) -> ChildNode<I> {
        Self {
            nodes: Box::new(vec![]),
            node_type: NodeType::Leaf(data),
            index_value,
        }
    }

    fn insert(&mut self, key: &[u8], data: &I) -> Result<(), ()> {
        // Check to see if the node exists in the children.
        // If it does recurse deeper.
        let key_len = key.len();
        let maybe_position = self.nodes.deref().iter().position(|n| {
            n.index_value == key[0]
        });

        if let Some(i) = maybe_position {
            if key_len > 1 {
                self.nodes[i]
                .insert(&key[1..], data)?;
            } else {
                // Leaf node exists, edit.
                self.nodes[i].node_type = NodeType::<I>::Leaf(data.clone());
            }
        } else {
        // Alas if we have not found a node then we must create our new recursive node and keep the existing data.
            if key_len > 1 {
                // Here we instantiate the new optional nodes and set the key to the new node.
                let mut new_node = ChildNode::new_branch(key[0]);
                // Again we must continue recursion on the new node until key is len 1.
                    new_node.insert(&key[1..], data)?;
                // This is done after recursion because we need that node to be populated with the nested nodes within it.
                self.nodes.push(new_node);
            } else {
                // Leaf node does not exit, create.
                let new_node = ChildNode::new_leaf( 
                    key[0],
                    data.clone(),
                );
                self.nodes.push(new_node);
            }
        }
        Ok(())
    }

    fn get(&self, key: &[u8]) -> Result<I, TrieError> {
        if let Some(pos) = &self.nodes.iter().position(|n| n.index_value == key[0]) {
            if key.len() > 1 {
                self.nodes[*pos].get(&key[1..])
            } else {
                match &self.nodes[*pos].node_type {
                    NodeType::Leaf(data) => Ok(data.clone()),
                    _ => Err(TrieError::ExpectedLeafGotBranch)
                }
            }
        } else {
            Err(TrieError::KeyDoesNotExist)
        }

    }
}

#[derive(Debug, PartialEq)]
pub enum NodeType<T> {
    Leaf(T),
    Branch,
}

/// The trie, currently programmed as base 16.
/// Uses the hash on the key inputted to compute a place in storage.
pub struct Trie<T: Digest, K: Sized, I: Sized + Clone> {
    pub root: ChildNode<I>,
    phantom_t: PhantomData<T>,
    phantom_k: PhantomData<K>,
}

// Where T is the hasher and I is the data.
// K is the key 
impl<T, K, I> Trie<T, K, I>
where
    T: Digest,
    K: Sized,
    I: Sized + Clone,
 {
    pub fn new() -> Self {
        Self {
            root: ChildNode::new_branch(0),
            phantom_k: PhantomData,
            phantom_t: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, data: I) -> Result<(), ()> {
        let hash_bytes = hash_me::<T, K>(key);
        // Compute the "decimal index representation of hex", a necessary evil for the behaivour of the hex trie .
        let index_representation = get_index_rep_of_hex(hash_bytes.as_slice());

        self.root.insert(
            index_representation.as_slice(),
            &data,
        )  
    }

    pub fn get(&self, key: K) -> Result<I, TrieError> {
        let hash_bytes = hash_me::<T, K>(key);

        // Compute the "decimal index representation of hex", a necessary evil for the behaivour of the hex trie .
        let index_of_hex = get_index_rep_of_hex(hash_bytes.as_slice());
        self.root.get(index_of_hex.as_slice())
    }

    fn _remove(_key: &[u8]) -> Result<(), ()> {


        Ok(())
    }

    // An idea to add patricia trie optimisation.
    fn _optimise_patricia(){
        todo!()
    }
}

// Helper Function
fn get_index_rep_of_hex(hash: &[u8]) -> Vec<u8> {
    hash
    .iter()
    .flat_map(|num| {
        // This will return the index related to the hex digit
        // Decimal | Hex | index
        // 255 = 0xff == 15,15,
        // 10 = 0x0A = 00,10,
        // 100 = 0x56 = 05,06, 
            decimal_to_hex_index(*num)
    }).collect::<Vec<u8>>()
}

 

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
    

#[test]
fn test_insert_and_retrieve_spam() {
    let mut trie: Trie<Blake2b512, f32, u64> = Trie::new();
    assert!(trie.insert(1000f32, 60u64).is_ok());
    let res = trie.get(1000f32);

    assert_eq!(res, Ok(60u64));
}


#[test]
fn test_retrive_nothing_errs() {
    let mut trie: Trie<Blake2b512, &str, f32> = Trie::new();
    assert!(trie.insert("hello", 60f32).is_ok());
    let res = trie.get("hello_world !!12345");
    assert_eq!(res, Err(TrieError::KeyDoesNotExist));
}

#[test]
fn test_hex() {


}