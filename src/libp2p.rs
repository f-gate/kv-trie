
extern crate alloc;

use libp2p::kad::{
    record::{
        store::RecordStore,
        Key
    },
    Record, ProviderRecord
};
use crate::trie::*;
use alloc::borrow::Cow;
use std::{iter, collections::{hash_map, hash_set}};

impl<'a ,T: StorageConfig> RecordStore<'a> for Trie<T> {
    //From libp2p repo: rust-libp2p/protocols/kad/src/record/store/memory.rs
    // Modified to used a Vec.
    type RecordsIter =
    iter::Map<hash_map::Values<'a, Key, Record>, fn(&'a Record) -> Cow<'a, Record>>;

    type ProvidedIter = iter::Map<
        hash_set::Iter<'a, ProviderRecord>,
        fn(&'a ProviderRecord) -> Cow<'a, ProviderRecord>,
    >;
   
    fn get(&self, k: &libp2p::kad::record::Key) -> Option<std::borrow::Cow<'_, libp2p::kad::Record>> {
        let result = <Self as StorageMethod<T>>::get(&self, k.to_vec().as_slice());

        None
    }

    fn put(&mut self, r: libp2p::kad::Record) -> libp2p::kad::store::Result<()> {
        todo!()
    }

    fn remove(&mut self, k: &libp2p::kad::record::Key) {
        todo!()
    }

    fn records(&self) -> Self::RecordsIter {
        todo!()
    }

    fn add_provider(&mut self, record: libp2p::kad::ProviderRecord) -> libp2p::kad::store::Result<()> {
        todo!()
    }

    fn providers(&self, key: &libp2p::kad::record::Key) -> Vec<libp2p::kad::ProviderRecord> {
        todo!()
    }

    fn provided(&self) -> Self::ProvidedIter {
        todo!()
    }

    fn remove_provider(&mut self, k: &libp2p::kad::record::Key, p: &libp2p::PeerId) {
        todo!()
    }
}


