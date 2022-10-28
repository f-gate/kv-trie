
    
    use crate::Trie;
    use blake2::Blake2b512;
    use std::time::Instant;
    
    pub fn bench_get() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        let _ = trie.insert("hello_world !! 12345", 60u64);
        let now = Instant::now();
    {
        let _ = trie.get("hello_world !! 12345");
    }
    
    let elapsed = now.elapsed();
        println!("Get() elapsed: {:.5?}", elapsed);
    }
        
    pub fn bench_insert() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        let now = Instant::now();
        {
            let _ = trie.insert("hello_world !! 12345", 60u64);
        }
        let elapsed = now.elapsed();
        println!("Insert() elapsed: {:.5?}", elapsed);

        let now = Instant::now();
        {
            let _ = trie.insert("hello_world !! 12345", 60u64);
        }
        let elapsed = now.elapsed();
        println!("Second insert to same key() elapsed: {:.5?}", elapsed);
    }


    pub fn insert_crazy() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        let max: u64 = 10_000;
        let input: Vec<String> = (0..max).into_iter().map(|i| {
            i.to_string()
        }).collect();
        let now = Instant::now();
        
        {
            (0..max).into_iter().map(|i| {

                let _ = trie.insert(input[i as usize].as_str(), max);
            }).collect::<_>()
            
        }
        let elapsed = now.elapsed();
        println!("insert crazy average elapsed: {:.5?}", elapsed/(max as u32));

    }
