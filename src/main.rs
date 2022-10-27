mod trie;
mod benchmarking;

use trie::*;


fn main() {
    benchmarking::bench_insert();
    benchmarking::bench_get();
    benchmarking::insert_crazy();
}


