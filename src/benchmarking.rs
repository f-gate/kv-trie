#![feature(test)]

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;


    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    #[bench]
    fn  bench_insert_1000(b: &mut Bencher) {
        b.iter(|| {
            // Use `test::black_box` to prevent compiler optimizations from disregarding
            // Unused values
            test::black_box (
                for c in 1..1000 {
                    trie.insert(c.to_string().as_str(), c);
                }
            );
        });
    }
}
