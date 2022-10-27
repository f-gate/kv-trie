
    
    use crate::Trie;
    use blake2::Blake2b512;
    
    pub fn bench_get() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        let _ = trie.insert("hello_world !! 12345", 60u64);
        use std::time::Instant;
        let now = Instant::now();
    {
        let _ = trie.get("hello_world !! 12345");
    }
    
    let elapsed = now.elapsed();
        println!("Get() elapsed: {:.2?}", elapsed);
    }
        
    pub fn bench_insert() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        use std::time::Instant;
        let now = Instant::now();
        {
            let _ = trie.insert("hello_world !! 12345", 60u64);
        }
        let elapsed = now.elapsed();
        println!("Insert() elapsed: {:.2?}", elapsed);

        let now = Instant::now();
        {
            let _ = trie.insert("hello_world !! 12345", 60u64);
        }
        let elapsed = now.elapsed();
        println!("Second insert to same key() elapsed: {:.2?}", elapsed);
    }


    pub fn insert_crazy() {
        let mut trie: Trie<Blake2b512, &str, u64> = Trie::new();
        use std::time::Instant;
        let now = Instant::now();
        let input: Vec<String> = (0..10000).into_iter().map(|i| {
            i.to_string()
        }).collect();
        {
            (0..10000u64).into_iter().map(|i| {

                let _ = trie.insert(input[i as usize].as_str(), 100000000u64);
            }).collect::<_>()
            
        }
        let elapsed = now.elapsed();
        println!("insert crazy average elapsed: {:.2?}", elapsed/10000u32);

    }
