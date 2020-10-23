/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use slonky::Slonky;
use slonky::ReadTx;
use slonky::WriteTx;

struct InMemorySlonky {

}

struct InMemoryReadTx {

}

struct InMemoryWriteTx {

}

impl Slonky for InMemorySlonky {
    fn read<T, E>(&self, f: Box<dyn Fn(Box<dyn ReadTx>) -> Result<T, E>  + Sync + Send>) -> Result<T, E> {
        unimplemented!()
    }

    fn write<E>(&self, f: Box<dyn Fn(Box<dyn WriteTx>) -> Result<(), E> + Sync + Send>) -> Result<(), E> {
        unimplemented!()
    }
}

impl ReadTx for InMemoryReadTx {
    fn key_exists(&self, key: &[u8]) -> bool {
        unimplemented!()
    }

    fn prefix_exists(&self, prefix: &[u8]) -> bool {
        unimplemented!()
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        unimplemented!()
    }

    fn prefix_scan(&self, prefix: &[u8]) -> Box<dyn Iterator<Item=(Vec<u8>, Vec<u8>)>> {
        unimplemented!()
    }

    fn range_scan(&self, from: &[u8], to: &[u8]) -> Box<dyn Iterator<Item=(Vec<u8>, Vec<u8>)>> {
        unimplemented!()
    }

    fn scan_all(&self) -> Box<dyn Iterator<Item=(Vec<u8>, Vec<u8>)>> {
        unimplemented!()
    }
}

impl WriteTx for InMemoryWriteTx {
    fn key_exists(&self, key: &[u8]) -> bool {
        unimplemented!()
    }

    fn prefix_exists(&self, prefix: &[u8]) -> bool {
        unimplemented!()
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        unimplemented!()
    }

    fn put(&self, key: &[u8], value: &[u8]) -> (Vec<u8>, Vec<u8>) {
        unimplemented!()
    }

    fn remove(&self, key: &[u8]) -> (Vec<u8>, Vec<u8>) {
        unimplemented!()
    }

    fn cancel(&self) {
        unimplemented!()
    }
}

pub fn create_in_memory_slonky() -> impl Slonky {
    InMemorySlonky {}
}

#[cfg(test)]
mod test;
