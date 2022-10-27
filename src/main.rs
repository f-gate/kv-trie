mod trie;
mod benchmarking;

use trie::*;
use blake2::Blake2b512;

fn main() {
    benchmarking::bench_insert();
    benchmarking::bench_get();
    benchmarking::insert_crazy();
}


