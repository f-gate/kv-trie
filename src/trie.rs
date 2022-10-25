use digest::*;
use blake2::*;
use core::marker::PhantomData;
use hex_literal::hex;
use digest::generic_array::functional::FunctionalSequence;
use std::mem::*;



#[derive(Debug)]
pub struct ChildNode<I: Sized + Clone> {
    value: u8,
    // Boxed due to recursive type.
    nodes: Box<[Option<ChildNode<I>>; 16]>,
    node_type: NodeType<I>,
}

impl<I: Sized + Clone> ChildNode<I> {
    // Create a empty which is of type branch.
    fn new(value: u8, maybe_type: Option<I>) -> ChildNode<I> {
        let mut node_type = NodeType::Branch;
        if maybe_type.is_some() {
            node_type = NodeType::Leaf(maybe_type.expect("is_some() is called above; qed"));
        }
        Self {
            value,
            nodes: Box::new([None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]),
            node_type
        }
    }

    fn insert(&mut self, key: &[u8], data: &I) -> Result<(), ()> {
        // Check to see if the node exists in the children.
        // If it does recurse deeper.
        let key_len = key.len(); 
        if  self.nodes[key[0] as usize].is_some() {

            if key_len > 1 {
                let _ = self.nodes[key[0] as usize]
                .as_mut()
                .expect("node has been found; qed")
                .insert(&key[1..], data)?;
            } else {
                self.nodes[key[0] as usize].as_mut().expect("node has been found; qed").node_type = NodeType::<I>::Leaf(data.clone());
                // leaf node exists, edit
            }
        } else {
        // Alas if we have not found a node then we must create our new recursive node and keep the existing data.
            if key_len > 1 {
                let mut new_node = ChildNode::new( 
                    key[0],
                    None
                );
                // Again we must continue recursion on the new node until key is len 1.
                    let _ = new_node.insert(&key[1..], data)?;
                // Here we instantiate the new optional nodes and set the key to the new node.
                // This is done after recursion because we need that node to be populated with the nested nodes within it.
                self.nodes[key[0] as usize] = Some(new_node);
            } else {
                // leaf node does not exit, create
                let new_node = ChildNode::new( 
                    key[0],
                    Some(data.clone())
                );
                self.nodes[key[0] as usize] = Some(new_node);
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum NodeType<T> {
    Leaf(T),
    Branch,
}

/// The trie, currently programmed as base 16.
/// Uses the hash on the key inputted to compute a place in storage.
pub struct Trie<T: Digest, I: Sized + Clone> {
    root: ChildNode<I>,
    phantom: PhantomData<T>,
}

// Where T is the hasher and I is the data. 
impl <T: Digest, I: Sized + Clone> Trie<T, I> {
    fn new() -> Self {
        Self {
            root: ChildNode::new(0, None),
            phantom: PhantomData,
        }
    }

    fn insert(&mut self, key: &str, data: I) -> Result<(), ()> {
        // Hash the key for better distribution.
        let mut hasher = T::new();
        hasher.update(key.as_bytes());
        let hash_bytes = hasher.finalize();

        // Compute the "decimal index representation of hex", a necessary evil for the behaivour of the hex trie .
        let index_representation = hash_bytes.as_slice()
        .iter()
        .flat_map(|num| {
            // This will return the index related to the hex digit
            // i.e 255d = 0xff == 15,15, 10d = 0x0A = 00,10, 100d = 0x56 = 05,06 
                decimal_to_hex_index(*num)
        }).collect::<Vec<u8>>();
        dbg!(&index_representation);

        // Each byte is a node.
        let sum: u16 =  index_representation.iter().map(|n|*n as u16).sum();
        self.root.insert(
        index_representation.as_slice(),
        &data,
    )  
    }

   

    fn get(&mut self, key: &[u8]) -> Result<(), ()> {

        todo!()
    }

    fn remove(key: &[u8]) -> Result<(), ()> {
        Ok(())
    }

    // An idea to add patricia trie optimisation.
    fn optimise_patricia() -> (){
        todo!()
    }
}


// for numbers below 255 only
fn decimal_to_hex_index(decimal: u8) -> [u8; 2] {
    [decimal / 16u8, decimal % 16u8]
}

#[test]
fn test_insert_state() {
    let mut trie: Trie<Blake2b512, u32> = Trie::new();
    assert!(trie.insert("hello_world !! 12345", 60u32).is_ok());
}

#[test]
fn test_hex() {


}