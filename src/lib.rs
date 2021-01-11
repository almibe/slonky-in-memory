// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use futures_core::stream::Stream;
use slonky::{Slonky, SlonkyError, SlonkyReadTx, SlonkyWriteTx};
use std::collections::BTreeMap;

struct SlonkyInMemory {
    data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl SlonkyInMemory {
    fn new() -> SlonkyInMemory {
        SlonkyInMemory {
            data: BTreeMap::new();
        }
    }
}

#[async_trait]
impl Slonky for SlonkyInMemory {
    async fn read(self) -> Result<Box<dyn SlonkyReadTx>, SlonkyError> {
        todo!()
    }

    async fn write(self) -> Result<Box<dyn SlonkyWriteTx>, SlonkyError> {
        todo!()
    }
}

struct SlonkyInMemoryReadTx {
    data: BTreeSet<Vec<u8>>,
}

#[async_trait]
impl SlonkyReadTx for SlonkyInMemoryReadTx {
    async fn key_exists(self, key: Vec<u8>) -> Result<bool, SlonkyError> {
        todo!()
    }

    async fn prefix_exists(self, prefix: Vec<u8>) -> Result<bool, SlonkyError> {
        todo!()
    }

    async fn get(self, key: Vec<u8>) -> Result<Option<Vec<u8>>, SlonkyError> {
        todo!()
    }

    async fn prefix_scan(self, prefix: Vec<u8>) -> Box<dyn Stream<Item = (Vec<u8>, Vec<u8>)>> {
        todo!()
    }

    async fn range_scan(
        self,
        from: Vec<u8>,
        to: Vec<u8>,
    ) -> Box<dyn Stream<Item = (Vec<u8>, Vec<u8>)>> {
        todo!()
    }

    async fn scan_all(self) -> Box<dyn Stream<Item = (Vec<u8>, Vec<u8>)>> {
        todo!()
    }
}

struct SlonkyInMemoryWriteTx {
    data: BTreeSet<Vec<u8>>,
}

#[async_trait]
impl SlonkyWriteTx for SlonkyInMemoryWriteTx {
    async fn key_exists(self, key: Vec<u8>) -> Result<bool, SlonkyError> {
        todo!()
    }

    async fn prefix_exists(self, prefix: Vec<u8>) -> Result<bool, SlonkyError> {
        todo!()
    }

    async fn get(self, key: Vec<u8>) -> Result<Option<Vec<u8>>, SlonkyError> {
        todo!()
    }

    async fn put(self, key: Vec<u8>, value: Vec<u8>) -> Result<(), SlonkyError> {
        todo!()
    }

    async fn remove(self, key: Vec<u8>) -> Result<(), SlonkyError> {
        todo!()
    }

    async fn cancel(self) -> Result<(), SlonkyError> {
        todo!()
    }
}
