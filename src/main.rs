
extern crate kv_trie;
use kv_trie::benchmarking;

fn main() {

    // Speed Benchmarks
    benchmarking::bench_get();
    benchmarking::bench_insert();
    benchmarking::insert_crazy();
    benchmarking::bench_removal_single();
    benchmarking::remove_crazy();

    // Storage Benchmarks
    benchmarking::count_number_of_branches();

}