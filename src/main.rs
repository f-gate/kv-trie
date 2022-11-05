mod trie;
mod benchmarking;
mod childnode;
mod utils;
use trie::*;


fn main() {
    println!("\n\nSpeed Benchmarks:");
    benchmarking::bench_insert();
    benchmarking::bench_get();
    benchmarking::insert_crazy();
    benchmarking::bench_removal_single();
    benchmarking::remove_crazy();

    println!("\nStorage Benchmarks:");
    benchmarking::count_number_of_branches();
    
    println!("\n");
}


