

//#[derive(Clone)]
pub struct ChildNode {
    value: u8,
    is_end: bool,
    nodes: Option<Box<[Option<ChildNode>; 16]>>
}

impl ChildNode {
    fn new(value: u8, is_end: bool) -> ChildNode {
        Self {
            value,
            is_end,
            nodes: None
        }
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

    fn insert(parent_node: &mut ChildNode, key: &[u8], data: &[u8]) -> Result<(), ()> {
        let mut has_found_node: bool = false;
        
        if let Some(nodes) = &mut parent_node.nodes {
            if key.len() != 1 {
                Self::insert(&mut nodes[key[0] as usize].as_ref().expect("node has been found; qed"), &key[1..], data);
            }
            has_found_node = true;
        }

        if !has_found_node {
            let mut new_node = ChildNode::new( 
                key[0],
                key.len() == 1
            );
            if key.len() != 1 {
                Self::insert(&mut new_node, &key[1..], data);
            }

            let mut nodes: [Option<ChildNode>; 16] = [None, None,None, None,None, None,None, None,None, None,None, None,None, None,None, None,];
            nodes[key[0] as usize] = Some(new_node);
            parent_node.nodes = Some(Box::new(nodes));
        }
        Ok(())
    }

    fn get(key: &[u8]) -> Result<(), ()> {
        Ok(())
    }

    fn remove(key: &[u8]) -> Result<(), ()> {
        Ok(())
    }
}
