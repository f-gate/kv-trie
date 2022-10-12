


use hex::*;

struct ChildNode {
    value: u8,
    is_end: bool,
    nodes: [Option<ChildNode>, 16]
};

impl ChildNode {
    fn new(value: u8, is_end: bool) {
        ChildNode {
            value,
            is_end,
            nodes: [None; 16]
        }
    }
}


struct Trie {
    root: ChildNode
};

impl Trie {
    fn new() -> Self {
        Self {
            root: ChildNode::new(0, false)
        }
    }

    fn insert(parent_node: &mut ChildNode, key: [u8], data: [u8]) -> Result<(), ()> {
        let has_found_node: bool = false;
        
        parent_node.nodes.iter().flatten().map(|n| {
            if n.value == key[0] {
                if key.len() != 1 {
                    insert(&mut n, key[1..], data)
                }
                has_found_node = true;
                break
            }
        })

        is !has_found_node {
            let mut new_node = ChildNode::new( 
                value: key[0],
                is_end: (key.len == 1)
            )
            if key.len() != 1 {
                insert(&mut new_node, key[1..], data)
                parent_node.nodes.push(new_node);
            } else {
                break
            }
            
        }

        Ok(())
    }

    fn get(key: Hash) -> Result<[u8], ()> {

        Ok([1])
    }

    fn remove(key: Hash) -> Result<(), ()> {
        Ok(())
    }
}

