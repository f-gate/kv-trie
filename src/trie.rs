

pub struct ChildNode {
    value: u8,
    is_end: bool,
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
        // check to see if the node exists in the children
        if  self.nodes[key[0] as usize].is_some() {
            if key.len() != 1 {
                self.nodes[key[0] as usize]
                .as_mut()
                .expect("node has been found; qed")
                .insert(&key[1..], data);
            } else {
                // set child node as false.
            }
        
            has_found_node = true;
        }

        if !has_found_node {
            let mut new_node = ChildNode::new( 
                key[0],
                key.len() == 1
            );
            if key.len() != 1 {
                new_node.insert(&key[1..], data);
            } else {
                // set child node as leaf
            }
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
