


use std::borrow::Cow;

use crate::utils::{NodeType, TrieError};


#[derive(Debug)]
pub struct ChildNode<I: Sized + Clone> {
    // Boxed due to recursive type.
    index_value: u8,
    pub nodes: Vec<ChildNode<I>>,
    pub node_type: NodeType<I>,
}

impl<I: Sized + Clone> ChildNode<I> {

    pub fn new_branch(index_value: u8) -> ChildNode<I> {
        Self {
            nodes: vec![],
            node_type: NodeType::Branch,
            index_value,
        }
    }
    pub fn new_leaf(index_value: u8, data: I) -> ChildNode<I> {
        Self {
            nodes: vec![],
            node_type: NodeType::Leaf(data),
            index_value,
        }
    }

    pub fn insert(&mut self, key: &[u8], data: &I) -> Result<(), ()> {
        // Check to see if the node exists in the children.
        let maybe_position = self.nodes.iter().position(|n| {
            n.index_value == key[0]
        });

        if let Some(i) = maybe_position {
            // Here we have found a node so we can recurse deeper.
            if key.len() > 1 {
                self.nodes[i]
                .insert(&key[1..], data)?;
            } else {
                // Leaf node exists, edit.
                self.nodes[i].node_type = NodeType::<I>::Leaf(data.clone());
            }
        } else {
            // Alas if we have not found a node then we must create our new recursive node and keep the existing data.
            if key.len() > 1 {
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

    pub fn get(&self, key: &[u8], is_owned: bool) -> Result<Cow<I>, TrieError> {
        if let Some(pos) = &self.nodes.iter().position(|n| n.index_value == key[0]) {
            if key.len() > 1 {
                // Continue recursion until we reach the leaf node.
                self.nodes[*pos].get(&key[1..], is_owned)
            } else {
                match &self.nodes[*pos].node_type {
                    NodeType::Leaf(data) => {
                        if is_owned {
                            Ok(Cow::Owned(data.clone()))
                        } else {
                            Ok(Cow::Borrowed(data))
                        }
                    },
                    _ => Err(TrieError::ExpectedLeafGotBranch)
                }
            }
        } else {
            Err(TrieError::KeyDoesNotExist)
        }
    }

    pub fn remove(&mut self, key: &[u8], key_len: usize) -> Result<bool, TrieError> {
        // recurse to leaf
        let mut is_removed = false;
        if let Some(pos) = &self.nodes.iter().position(|n| n.index_value == key[0]) {
            if key.len() > 1 {
                // Recurse to the leaf node and instantiate the is_removed for removal.
                is_removed = self.nodes[*pos].remove(&key[1..], key_len)?;
            } else {
                match &self.nodes[*pos].node_type {
                    NodeType::Leaf(_) => {
                        // Push onto res for deletion
                        return Ok(false)
                    },
                    _ => return Err(TrieError::ExpectedLeafGotBranch)
                }
            }
            // We are looking for a node which has more than one childnode.
            // This is because that is the highest place we can remove the node.
            // A is_removed is used to mark if this highest node has been found as to not remove again.
            // Also the edge case is caught where there is always one node per branch.
            if self.nodes.len() == 1usize  {
                // Here we catch the edge case where we are at the root of the branch.
                if key.len() == key_len && !is_removed  {
                    let _ = self.nodes.remove(*pos);
                    is_removed = true;
                }
            } else {
                // Here we have found a node with more than one child so we can delete that node.
                if !is_removed {
                    // drop the node from there.
                    let _ = self.nodes.remove(*pos);
                    is_removed = true;
                }
            }
        } else {
            return Err(TrieError::KeyDoesNotExist);
        }
        Ok(is_removed)
    }

}
