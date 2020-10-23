/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use slonky::Slonky;
use slonky::ReadTx;
use slonky::WriteTx;
use futures::stream::Stream;
use async_trait::async_trait;

struct InMemorySlonky {

}

struct InMemoryReadTx {

}

struct InMemoryWriteTx {

}

#[async_trait]
impl Slonky for InMemorySlonky {
    async fn read<T, E>(&self, f: Box<dyn Fn(dyn ReadTx) -> Result<T, E>  + Sync + Send>) -> Result<T, E> {
        unimplemented!()
    }

    async fn write<E>(&self, f: Box<dyn Fn(dyn WriteTx) -> Result<(), E> + Sync + Send>) -> Result<(), E> {
        unimplemented!()
    }
}

#[async_trait]
impl ReadTx for InMemoryReadTx {
    async fn key_exists(&self, key: &[u8]) -> bool {
        unimplemented!()
    }

    async fn prefix_exists(&self, prefix: &[u8]) -> bool {
        unimplemented!()
    }

    async fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        unimplemented!()
    }

    async fn prefix_scan(&self, prefix: &[u8]) -> Box<dyn Stream<Item=(Vec<u8>, Vec<u8>)>> {
        unimplemented!()
    }

    async fn range_scan(&self, from: &[u8], to: &[u8]) -> Box<dyn Stream<Item=(Vec<u8>, Vec<u8>)>> {
        unimplemented!()
    }

    async fn scan_all(&self) -> Box<dyn Stream<Item=(Vec<u8>, Vec<u8>)>> {
        unimplemented!()
    }
}

#[async_trait]
impl WriteTx for InMemoryWriteTx {
    async fn key_exists(&self, key: &[u8]) -> bool {
        unimplemented!()
    }

    async fn prefix_exists(&self, prefix: &[u8]) -> bool {
        unimplemented!()
    }

    async fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        unimplemented!()
    }

    async fn put(&self, key: &[u8], value: &[u8]) -> (Vec<u8>, Vec<u8>) {
        unimplemented!()
    }

    async fn remove(&self, key: &[u8]) -> (Vec<u8>, Vec<u8>) {
        unimplemented!()
    }

    async fn cancel(&self) {
        unimplemented!()
    }
}

// pub fn create_in_memory_slonky() -> impl Slonky {
//     unimplemented!()
// }

#[cfg(test)]
mod test;
