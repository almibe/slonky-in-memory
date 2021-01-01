/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::BTreeSet;
use slonky::{Slonky, SlonkyError, SlonkyReadTx, SlonkyWriteTx};
use async_trait::async_trait;
use futures_core::stream::Stream;

struct InMemorySlonky {
  data: BTreeSet<Vec<u8>>
}

#[async_trait]
impl Slonky for InMemorySlonky {
  async fn read(self) -> Result<Box<dyn SlonkyReadTx>, SlonkyError> {
    todo!()
  }

  async fn write(self) -> Result<Box<dyn SlonkyWriteTx>, SlonkyError> {
    todo!()
  }
}

struct InMemorySlonkyReadTx {
  data: BTreeSet<Vec<u8>>
}

#[async_trait]
impl SlonkyReadTx for InMemorySlonkyReadTx {
  async fn key_exists(self, key: Vec<u8>) -> Result<bool, SlonkyError> {
    todo!()
  }

  async fn prefix_exists(self, prefix: Vec<u8>) -> Result<bool, SlonkyError> {
    todo!()
  }

  async fn get(self, key: Vec<u8>) -> Result<Option<Vec<u8>>, SlonkyError> {
    todo!()
  }

  fn prefix_scan(self, prefix: Vec<u8>) -> Box<dyn Stream<Item = (Vec<u8>, Vec<u8>)>> {
    todo!()
  }

  fn range_scan(self, from: Vec<u8>, to: Vec<u8>) -> Box<dyn Stream<Item = (Vec<u8>, Vec<u8>)>> {
    todo!()
  }

  fn scan_all(self) -> Box<dyn Stream<Item = (Vec<u8>, Vec<u8>)>> {
    todo!()
  }
}

struct InMemorySlonkyWriteTx {
  data: BTreeSet<Vec<u8>>
}

#[async_trait]
impl SlonkyWriteTx for InMemorySlonkyWriteTx {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
