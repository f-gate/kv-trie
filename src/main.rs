mod trie;
mod benchmarking;

use trie::*;


fn main() {
    println!("\n\nSpeed Benchmarks:");
    benchmarking::bench_insert();
    benchmarking::bench_get();
    benchmarking::insert_crazy();

    println!("\nStorage Benchmarks:");
    benchmarking::count_number_of_branches();
    
    println!("\n");
}


