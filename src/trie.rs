
#[derive(Debug)]
pub struct ChildNode {
    value: u8,
    // Boxed due to infinate recursion.
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
    }
        Ok(())
}

pub enum NodeType {
    Leaf,
    Branch,
}

pub struct Trie {
    root: ChildNode
}

impl Trie {
    fn new() -> Self {
        Self {
            root: ChildNode::new(0, false)
        }
    }

    fn insert(&mut self, key: &[u8], data: &[u8]) -> Result<(), ()> {
        self.root.insert(key, data)
    }
   

    fn get(key: &[u8]) -> Result<(), ()> {
        Ok(())
    }

    fn remove(key: &[u8]) -> Result<(), ()> {
        Ok(())
    }

    // An idea to add patricia trie optimisation.
    fn optimise_patricia() -> (){
        todo!()
    }
}


// current issues is that when inserting 122 it overrides 123
#[test]
fn test_insert_state() {
    let mut Trie = Trie::new();

    assert!(Trie.insert(&[1,2,3], &[0]).is_ok());
    assert!(Trie.insert(&[1,2,2,3], &[0]).is_ok());
    dbg!(&Trie.root);

    assert!(Trie.insert(&[1,2], &[0]).is_err());

}