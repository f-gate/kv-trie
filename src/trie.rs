


use hex::*;

#[derive(Clone)]
pub struct ChildNode {
    value: u8,
    is_end: bool,
    nodes: Option<Box<[ChildNode; 16]>>
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

    fn insert(parent_node: &ChildNode, key: &[u8], data: &[u8]) -> Result<(), ()> {
        let mut has_found_node: bool = false;
        
        parent_node.nodes.as_deref().into_iter().flatten().map(|mut n| {
            if n.value == key[0] {
                if key.len() != 1 {
                    Self::insert(n, &key[1..], data);
                }
                has_found_node = true;
            }
            ()
        }).collect::<()>();

        if !has_found_node {
            let mut new_node = ChildNode::new( 
                key[0],
                key.len() == 1
            );
            if key.len() != 1 {
                Self::insert(&new_node, &key[1..], data);
            }

            //parent_node.nodes = Some(Box::new(new_node))
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
