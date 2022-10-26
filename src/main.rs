mod trie;
mod benchmarking;

use trie::*;
use blake2::Blake2b512;

fn main() {
    bench_insert();
    bench_get();
}

  
fn bench_get() {
    let mut trie: Trie<Blake2b512, u64, &str> = Trie::new();
    trie.insert("hello_world !! 12345", 60u64);

    use std::time::Instant;
        let now = Instant::now();

{
    trie.get("hello_world !! 12345");
}

let elapsed = now.elapsed();
    println!("Get() elapsed: {:.2?}", elapsed);
}


    
    fn bench_insert() {
        let mut trie: Trie<Blake2b512, u64, &str> = Trie::new();

        use std::time::Instant;
            let now = Instant::now();

    {
        trie.insert("hello_world !! 12345", 60u64);
    }

    let elapsed = now.elapsed();
        println!("Insert() elapsed: {:.2?}", elapsed);
    }
