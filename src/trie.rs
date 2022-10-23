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

    fn insert(&mut self, key: &[u8], data: &[u8]) -> Result<(), ()> {

        // Check to see if the node exists in the children.
        if  self.nodes[key[0] as usize].is_some() {
            // If the key is not at the end then see if we can find a deeper node.
            if key.len() != 1 {
                let _ = self.nodes[key[0] as usize]
                .as_mut()
                .expect("node has been found; qed")
                .insert(&key[1..], data)?;
            } else {
                // We have found the leaf node and must go no further. 
            }
        } else {
        // Alas if we have not found a node then we must create our new recursive node and keep the existing data.
            let mut new_node = ChildNode::new( 
                key[0],
                key.len() == 1
            );
            // Again we must continue recursion on the new node until key is len 1.
            if key.len() != 1 {
                let _ = new_node.insert(&key[1..], data)?;
            }
            // Here we instantiate the new optional nodes and set the key to the new node.
            // This is done after recursion because we need that node to be populated with the nested nodes within it.
            self.nodes[key[0] as usize] = Some(new_node);
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

        // Get the complete binary as a vec of bits.
        let bits = hasher.finalize().as_slice()
        .iter()
        .flat_map(|num| {
            // This will return the index related to the hex digit
            // i.e 255d = 0xff == 15,15, 10d = 0x0A = 00,10, 100d = 0x56 = 05,06 
                decimal_to_hex_index(*num)
        }).collect::<Vec<u8>>();
        dbg!(&bits);
        // Each byte is a node.
        self.root.insert(bits.as_slice(), data)  
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


// for numbers below 128 only
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