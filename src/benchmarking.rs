
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Trie;
    use blake2::Blake2b512;

    #[test]
    fn bench_add_two() {
        let mut trie: Trie<Blake2b512, u64, &str> = Trie::new();

        use std::time::Instant;
            let now = Instant::now();

    {
        trie.insert("hello_world !! 12345", 60u64);
    }

    let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }

}
