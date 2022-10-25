use digest::*;
use blake2::*;
use core::marker::PhantomData;
use hex_literal::hex;
use digest::generic_array::functional::FunctionalSequence;



#[derive(Debug)]
pub struct ChildNode {
    value: u8,
    // Boxed due to recursive type.
    nodes: Box<[Option<ChildNode>; 16]>,
    node_type: NodeType,
}

impl ChildNode {
    // Create a empty which is of type branch.
    fn new(value: u8, is_end: bool) -> ChildNode {
        Self {
            value,
            nodes: Box::new([None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]),
            node_type: if !is_end {NodeType::Branch} else {NodeType::Leaf} ,
        }
    }

    fn insert(&mut self, key: &[u8], data: &[u8], hash_index: &u16) -> Result<(), ()> {

        // the majority of the time the key length will not be 1 so ive put it as the primary condition.
        if key.len() != 1 {

            // Check to see if the node exists in the children.
            // If it does recurse deeper.
            if  self.nodes[key[0] as usize].is_some() {
                    let _ = self.nodes[key[0] as usize]
                    .as_mut()
                    .expect("node has been found; qed")
                    .insert(&key[1..], data, hash_index)?;
            } else {
            // Alas if we have not found a node then we must create our new recursive node and keep the existing data.
                let mut new_node = ChildNode::new( 
                    key[0],
                    false
                );
                // Again we must continue recursion on the new node until key is len 1.
                    let _ = new_node.insert(&key[1..], data, hash_index)?;
                // Here we instantiate the new optional nodes and set the key to the new node.
                // This is done after recursion because we need that node to be populated with the nested nodes within it.
                self.nodes[key[0] as usize] = Some(new_node);
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum NodeType {
    Leaf,
    Branch,
}

pub struct Trie<T: Digest> {
    root: ChildNode,
    phantom: PhantomData<T>,
}

impl <T: Digest> Trie<T> {
    fn new() -> Self {
        Self {
            root: ChildNode::new(0, false),
            phantom: PhantomData,
        }
    }

    fn insert(&mut self, key: &str, data: &[u8]) -> Result<(), ()> {
        // Hash the key for better distribution.
        let mut hasher = T::new();
        hasher.update(key.as_bytes());
        let hash_bytes = hasher.finalize();

        // Compute the "decimal index representation of hex", an evil thing.
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
        data,
        &sum
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
    let mut trie: Trie<Blake2b512> = Trie::new();
    assert!(trie.insert("hello_world !! 12345", &[0]).is_ok());
}

#[test]
fn test_hex() {


}