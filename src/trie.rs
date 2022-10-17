

pub struct ChildNode {
    value: u8,
    is_end: bool,
    // Boxed due to infinate recursion.
    nodes: Box<[Option<ChildNode>; 16]>
}

impl ChildNode {
    fn new(value: u8, is_end: bool) -> ChildNode {
        Self {
            value,
            is_end,
            nodes: Box::new([None, None,None, None,None, None,None, None,None, None,None, None,None, None,None, None])
        }
    }

    fn insert(&mut self, key: &[u8], data: &[u8]) -> Result<(), ()> {
        let mut has_found_node: bool = false;

        // Check to see if the node exists in the children.
        if  self.nodes[key[0] as usize].is_some() {
            // If the key is not at the end then see if we can find a deeper node.
            if key.len() != 1 {
                let _ = self.nodes[key[0] as usize]
                .as_mut()
                .expect("node has been found; qed")
                .insert(&key[1..], data)?;
            } else {
                // Set child node as leaf.
            }
            // This is required so after recursion is complete we don't override the existing data.
            has_found_node = true;
        }

        // Alas if we have not found a node then we must create a new node full of None bar the node we need.
        if !has_found_node {
            let mut new_node = ChildNode::new( 
                key[0],
                key.len() == 1
            );
            // Again we must continue recursion on the new node until key is len 1.
            if key.len() != 1 {
                let _ = new_node.insert(&key[1..], data)?;
            } else {
                // set child node as leaf.
            }
            // Here we instantiate the new optional nodes and set the key to the new node.
            // This is done after recursion because we need that node to be populated with the nested nodes within it.
            let mut nodes: [Option<ChildNode>; 16] = [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,];
            nodes[key[0] as usize] = Some(new_node);
            self.nodes = Box::new(nodes);
        }

        Ok(())
    }
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
}
