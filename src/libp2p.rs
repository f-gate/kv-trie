/// An example on how to integrate your own data structure into libp2p's Kademlia protocol.
/// This Memory store trait is passed into the Kademlia which in turn is used
/// when instantiating the swarm.
/// Unfortunatly the methods provided by RecordStore are not very generic.

/// I am leaving this to move onto libp2p some more. A nice poc.

extern crate alloc;
use core::hash::Hash;
use libp2p::kad::{
    record::{
        store::RecordStore,
        Key
    },
    Record, ProviderRecord
};
use crate::trie::*;
use alloc::borrow::Cow;
use std::{iter, collections::{vec_deque,}};
use digest::Digest;
/// Implementing RecordStore for my generic Trie which is bound
/// to the input strucs are that are defined within RecordStore. 
impl <'a, T, K, I>  RecordStore<'a> for Trie<T, K, I> 
where 
    T: Sized + Clone + Hash + Eq + Default,
    K: Sized + Clone + Hash + Eq,
    I:  Digest,
    Trie<T, K, I>: StorageMethod<Record, Vec<u8>, I>
{
    type RecordsIter = iter::Map<
        vec_deque::Iter<'a, Record>,
        fn(&'a Record) -> Cow<'a, Record>,
    >;

    type ProvidedIter = iter::Map<
        vec_deque::Iter<'a, ProviderRecord>,
        fn(&'a ProviderRecord) -> Cow<'a, ProviderRecord>,
    >;
   
    fn get(&self, k: &libp2p::kad::record::Key) -> Option<std::borrow::Cow<'_, libp2p::kad::Record>> {
        let result = <Self as StorageMethod<Record, Vec<u8>, I>>::get(&self, k.to_vec(), false);
        if let Ok(data) = result {
            Some(data)
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
        //<Self as StorageMethod<Record, Vec<u8>, I>>::get_records_iter(&self).iter().map(|R| Cow::Owned(R.clone()))
        todo!()
    }

    fn add_provider(&mut self, record: libp2p::kad::ProviderRecord) -> libp2p::kad::store::Result<()> {
        Ok(())
    }

    fn providers(&self, key: &libp2p::kad::record::Key) -> Vec<libp2p::kad::ProviderRecord> {
        vec![]
    }

    fn provided(&self) -> Self::ProvidedIter {
        todo!();
    }

    fn remove_provider(&mut self, k: &libp2p::kad::record::Key, p: &libp2p::PeerId) {
        ()
    }
}
