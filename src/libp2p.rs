/// An example on how to integrate your own data structure into libp2p's Kademlia protocol.
/// This Memory store trait is passed into the Kademlia which in turn is used
/// when instantiating the swarm.
/// Unfortunatly the methods provided by RecordStore are not very generic and require
/// alot of fiddeling when using a generic heavy structure like my trie.

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
use std::{iter, collections::{hash_map, hash_set, HashMap, HashSet}};
use digest::Digest;
/// Implementing RecordStore for my generic Trie which is bound
/// to the input strucs are that are defined within RecordStore. 
impl <'a, T: Sized + Clone, K: Sized + Clone, I: Digest>  RecordStore<'a> for Trie<T, K, I> 
where Trie<T, K, I>: StorageMethod<Record, Vec<u8>, I>
{
    //From libp2p repo: rust-libp2p/protocols/kad/src/record/store/memory.rs
    type RecordsIter =
    iter::Map<hash_map::Values<'a, Key, Record>, fn(&'a Record) -> Cow<'a, Record>>;

    type ProvidedIter = iter::Map<
        hash_set::Iter<'a, ProviderRecord>,
        fn(&'a ProviderRecord) -> Cow<'a, ProviderRecord>,
    >;
   
    fn get(&self, k: &libp2p::kad::record::Key) -> Option<std::borrow::Cow<'_, libp2p::kad::Record>> {
        let result = <Self as StorageMethod<Record, Vec<u8>, I>>::get(&self, k.to_vec());
        if let Ok(data) = result {
            Some(Cow::Owned(
                Record {
                    key: k.clone(),
                    value: data.value,
                    publisher: None,
                    expires: None,
                    }
                )
            )
        } else {
            None
        }
    }

    fn put(&mut self, r: libp2p::kad::Record) -> libp2p::kad::store::Result<()> {
        let _ = <Self as StorageMethod<Record, Vec<u8>, I>>::insert(self, r.key.to_vec(), r);
        Ok(())
    }

    fn remove(&mut self, k: &libp2p::kad::record::Key) {
        let _ =  <Self as StorageMethod<Record, Vec<u8>, I>>::remove(self, k.to_vec());
        ()
    }


    fn records(&self) -> Self::RecordsIter {
        HashMap::<Key, Record>::new().values().map(Cow::Borrowed)
    }

    fn add_provider(&mut self, record: libp2p::kad::ProviderRecord) -> libp2p::kad::store::Result<()> {
        Ok(())
    }

    fn providers(&self, key: &libp2p::kad::record::Key) -> Vec<libp2p::kad::ProviderRecord> {
        vec![]
    }

    fn provided(&self) -> Self::ProvidedIter {
        HashSet::<ProviderRecord>::new().iter().map(Cow::Borrowed)
    }

    fn remove_provider(&mut self, k: &libp2p::kad::record::Key, p: &libp2p::PeerId) {
        ()
    }
}
