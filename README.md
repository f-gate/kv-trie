# kv-trie

This is the start of a larger distributed project yet to be named.

None if it is meant to be exceptionally quick or efficient but i will try to maintain a sense of the aesthetic.



```
V2 benchmarks 30/10/22  {
    Speed Benchmarks:
        Insert elapsed: 111.36µs
        Second insert to same key() elapsed: 43.90µs
        Get elapsed: 41.42µs
        Insert 10000 items average elapsed: 67.53µs

    Storage Benchmarks:
        122_682_717 branches,  1_000_000 leafs. After 1000000 inserts
        2968.405 Mb used space.
        0 wasted as option arrays were removed :3.
        performance decreases for insert increases for get.
}



 V1 benchmarks 30/10/22 { 
    Speed Benchmarks {
        Insert elapsed: 104.05µs
        Second insert to same key() elapsed: 36.46µs
        Get elapsed: 65.64µs
        Insert 10000 items average elapsed: 52.77µs
    }
    
    Storage Benchmarks {
        123_683_540 is_some,  1_855_253_116 is_none. After 1000000 inserts
        2968.405 Mb used space
        44_526.08 Mb wasted space
        93.75% wasted 
    }
}
