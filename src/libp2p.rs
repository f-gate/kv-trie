/// An example on how to integrate your own data structure into libp2p's Kademlia protocol.
/// This Memory store trait is passed into the Kademlia which in turn is used
/// when instantiating the swarm.
/// Unfortunatly the methods provided by RecordStore are not very generic and require
/// alot of fiddeling when using a generic heay structure like my trie.
/// In some cases it seems neccesary to remove much of my generic implementation all together :(

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
use blake2::{self, Blake2b512};
use digest::Digest;

impl <'a, T: Sized + Clone>  RecordStore<'a> for Trie<T> 
where Trie<T>: StorageMethod<Record, Vec<u8>, Blake2b512>
{
    //From libp2p repo: rust-libp2p/protocols/kad/src/record/store/memory.rs
    // Modified to used a Vec.
    type RecordsIter =
    iter::Map<hash_map::Values<'a, Key, Record>, fn(&'a Record) -> Cow<'a, Record>>;

    type ProvidedIter = iter::Map<
        hash_set::Iter<'a, ProviderRecord>,
        fn(&'a ProviderRecord) -> Cow<'a, ProviderRecord>,
    >;
   
    fn get(&self, k: &libp2p::kad::record::Key) -> Option<std::borrow::Cow<'_, libp2p::kad::Record>> {
//
        let result = <Self as StorageMethod<Record, Vec<u8>, Blake2b512>>::get(&self, k.to_vec());
        if let Ok(data) = result {
            Some(Cow::Owned(
                Record {
                    key: k.clone(),
                    //todo
                    value: vec![],
                    publisher: None,
                    expires: None,
                    }
                )
            )
        } else {
            // Handle Error?
            None
        }
    }

    fn put(&mut self, r: libp2p::kad::Record) -> libp2p::kad::store::Result<()> {

        <Self as StorageMethod<Record, Vec<u8>, Blake2b512>>::insert(self, r.key.to_vec(), r);

        return Ok(())
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
