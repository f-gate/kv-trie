
    
    use crate::{Trie, utils::*, childnode::*};
    use std::time::Instant;
    use blake2::Blake2b512;


    pub fn bench_get() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        let _ = trie.insert("hello_world !! 12345", 60u64);
        let now = Instant::now();
    {
        let _ = trie.get("hello_world !! 12345");
    }
    
    let elapsed = now.elapsed();
        println!("Get elapsed: {:.2?}", elapsed);
    }
        
    pub fn bench_insert() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        let now = Instant::now();
        {
            let _ = trie.insert("hello_world !! 12345", 60u64);
        }
        let elapsed = now.elapsed();
        println!("Insert elapsed: {:.2?}", elapsed);

        let now = Instant::now();
        {
            let _ = trie.insert("hello_world !! 12345", 60u64);
        }
        let elapsed = now.elapsed();
        println!("Second insert to same key() elapsed: {:.5?}", elapsed);
    }


    pub fn insert_crazy() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        let num = 10_000u64;
        let input: Vec<String> = (0..num).into_iter().map(|i| {
            i.to_string()
        }).collect();
        let now = Instant::now();
        {
            (0..num).into_iter().map(|i| {

                let _ = trie.insert(input[i as usize].as_str(), num);
            }).collect::<_>()
            
        }
        let elapsed = now.elapsed();
        println!("Insert {} items average elapsed: {:.2?}", num,  elapsed/num as u32);

    }

    pub fn count_number_of_branches() {
        let num = 10_000u64;
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
          let input: Vec<String> = (0..num).into_iter().map(|i| {
            i.to_string()
        }).collect();

        let _ = (0..num).into_iter().map(|i| {
                let _ = trie.insert(input[i as usize].as_str(), num);
        }).collect::<()>();

        let _size = (std::mem::size_of::<ChildNode<u64>>() as f32 / 1_000_000.0) as f32;
        let res = recurse_to_find_branches(&trie.root);
        
        println!( "~ {} branches, {} leafs. After {} inserts.", res.0, res.1, num);
    }

    // .0 branch .1 leaf.
    fn recurse_to_find_branches(trie: &ChildNode<u64>) -> (u64, u64) {
        let mut branch_count: (u64, u64) = (0u64, 0u64);
        for i in 0..16usize {
            if let Some(node) = trie.nodes.get(i)  {
                if node.node_type == NodeType::Branch {
                    branch_count.0 += 1;
                    let (nodes, leafs) = recurse_to_find_branches(node);
                    branch_count.0 += nodes;
                    branch_count.1 += leafs;
                } else {
                    branch_count.1 += 1;
                } 
            }
        }
        branch_count
    }


    pub fn bench_removal_single() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        let _ = trie.insert("hello_world !! 12345", 60u64);
        use std::time::Instant;
        let now = Instant::now();
    {
        let _ = trie.remove("hello_world !! 12345");
    }
        let elapsed = now.elapsed();
        println!("Remove elapsed: {:.2?}", elapsed);
    }

    pub fn remove_crazy() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        use std::time::Instant;
        let num = 10_000u64;
        let input: Vec<String> = (0..num).into_iter().map(|i| {
            i.to_string()
        }).collect();
            let _ =  (0..num).into_iter().map(|i| {

                let _ = trie.insert(input[i as usize].as_str(), num);
            }).collect::<()>();
   
            let now = Instant::now();
        {
            (0..num).into_iter().map(|i| {

                let _ = trie.remove(input[i as usize].as_str());
            }).collect::<_>()
            
        }
        let elapsed = now.elapsed();
        println!("Remove {} items average elapsed: {:.2?}", num,  elapsed/num as u32);

    }
