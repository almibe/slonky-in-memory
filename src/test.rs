/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::create_in_memory_slonky;
use slonky::ReadTx;
use slonky::Slonky;

fn to_list(fut: &dyn Iterator<Item=(Vec<u8>, Vec<u8>)>) -> Vec<u8> {
    unimplemented!()
}

#[test]
fn test_instances_should_start_empty() {
    let slonky = create_in_memory_slonky();
    let res = slonky.read::<Vec<u8>, String>(
        Box::new(|tx: Box<dyn ReadTx>| Result::Ok(to_list(&tx.scan_all())))
    );
    assert!(res.expect("").is_empty())
}

// #[test]
// fn test_test_adding_a_pair() {
//     let slonky = create_in_memory_slonky();
//     slonk.write. tx =>
//         tx.put(hex"ACAB", hex"DEAD")
//         }
//                                           all <- instance.read.use { tx =>
//         tx.scanAll().toListL
//         }
//                                           } yield all
//     }.toSet
//     assertEquals(res, Set((hex"ACAB", hex"DEAD")))
// }
//
// #[test]
// fn test_test_removing_a_pair() {
//     let slonky = create_in_memory_slonky() { instance =>
//                                           for {
//                                           _ <- instance.write.use { tx =>
//         tx.put(hex"ACAB", hex"DEAD")
//         }
//                                           _ <- instance.write.use { tx =>
//         tx.remove(hex"ACAB")
//         }
//                                           all <- instance.read.use { tx =>
//         tx.scanAll().toListL
//         }
//                                           } yield all
//     }.toSet
//     assert(res.isEmpty)
// }
//
// #[test]
// fn test_test_prefixing_in_readtx() {
//     let slonky = create_in_memory_slonky() { instance =>
//                                           for {
//                                           _ <- instance.write.use { tx =>
//         for {
//         _ <- tx.put(hex"ACAB", hex"DEAD")
//         _ <- tx.put(hex"ACAC", hex"BEEF")
//         _ <- tx.put(hex"CAAB", hex"DEAD")
//         } yield ()
//         }
//                                           prefixExists <- instance.read.use { tx =>
//         tx.prefixExists(hex"AC")
//         }
//                                           prefixScan <- instance.read.use { tx =>
//         tx.prefixScan(hex"AC").toListL
//         }
//                                           } yield (prefixExists, prefixScan)
//     }
//     assert(res._1)
//     assertEquals(res._2.toSet, Set((hex"ACAB", hex"DEAD"), (hex"ACAC", hex"BEEF")))
// }
//
// #[test]
// fn test_ranges() {
//     let slonky = create_in_memory_slonky() { instance =>
//                                           for {
//                                           _ <- instance.write.use { tx =>
//         for {
//         _ <- tx.put(hex"ACAB", hex"DEAD")
//         _ <- tx.put(hex"ACAC", hex"BEEF")
//         _ <- tx.put(hex"ADD", hex"DDDD")
//         _ <- tx.put(hex"CAAB", hex"DEAD")
//         } yield ()
//         }
//                                           rangeScan <- instance.read.use { tx =>
//         tx.rangeScan(hex"AC", hex"C").toListL
//         }
//                                           } yield rangeScan
//     }
//     assertEquals(res.toSet, Set((hex"ACAB", hex"DEAD"), (hex"ACAC", hex"BEEF"), (hex"ADD", hex"DDDD")))
// }
//
// #[test]
// fn test_canceling_writes() {
//     let slonky = create_in_memory_slonky() { instance =>
//                                           for {
//                                           _ <- instance.write.use { tx =>
//         for {
//         _ <- tx.put(hex"ACAB", hex"DEAD")
//         _ <- tx.put(hex"ACAC", hex"BEEF")
//         } yield ()
//         }
//                                           _ <- instance.write.use { tx =>
//         for {
//         _ <- tx.put(hex"ADD", hex"DDDD")
//         _ <- tx.put(hex"CAAB", hex"DEAD")
//         _ <- tx.cancel()
//         } yield ()
//         }
//                                           all <- instance.read.use { tx =>
//         tx.scanAll().toListL
//         }
//                                           } yield all
//     }
//     assertEquals(res.toSet, Set((hex"ACAB", hex"DEAD"), (hex"ACAC", hex"BEEF")))
// }
