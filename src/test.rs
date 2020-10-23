/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[test]
fn test_instances_should_start_empty() {
    val res = createSlonky.instance.use { instance =>
                                          instance.read.use { tx =>
        tx.scanAll().toListL
                                          }
    }.runSyncUnsafe()
    assert(res.isEmpty)
}

#[test]
fn test_test_adding_a_pair() {
    val res = createSlonky.instance.use { instance =>
                                          for {
                                          _ <- instance.write.use { tx =>
        tx.put(hex"ACAB", hex"DEAD")
        }
                                          all <- instance.read.use { tx =>
        tx.scanAll().toListL
        }
                                          } yield all
    }.runSyncUnsafe().toSet
    assertEquals(res, Set((hex"ACAB", hex"DEAD")))
}

#[test]
fn test_test_removing_a_pair() {
    val res = createSlonky.instance.use { instance =>
                                          for {
                                          _ <- instance.write.use { tx =>
        tx.put(hex"ACAB", hex"DEAD")
        }
                                          _ <- instance.write.use { tx =>
        tx.remove(hex"ACAB")
        }
                                          all <- instance.read.use { tx =>
        tx.scanAll().toListL
        }
                                          } yield all
    }.runSyncUnsafe().toSet
    assert(res.isEmpty)
}

#[test]
fn test_test_prefixing_in_readtx() {
    val res = createSlonky.instance.use { instance =>
                                          for {
                                          _ <- instance.write.use { tx =>
        for {
        _ <- tx.put(hex"ACAB", hex"DEAD")
        _ <- tx.put(hex"ACAC", hex"BEEF")
        _ <- tx.put(hex"CAAB", hex"DEAD")
        } yield ()
        }
                                          prefixExists <- instance.read.use { tx =>
        tx.prefixExists(hex"AC")
        }
                                          prefixScan <- instance.read.use { tx =>
        tx.prefixScan(hex"AC").toListL
        }
                                          } yield (prefixExists, prefixScan)
    }.runSyncUnsafe()
    assert(res._1)
    assertEquals(res._2.toSet, Set((hex"ACAB", hex"DEAD"), (hex"ACAC", hex"BEEF")))
}

#[test]
fn test_ranges() {
    val res = createSlonky.instance.use { instance =>
                                          for {
                                          _ <- instance.write.use { tx =>
        for {
        _ <- tx.put(hex"ACAB", hex"DEAD")
        _ <- tx.put(hex"ACAC", hex"BEEF")
        _ <- tx.put(hex"ADD", hex"DDDD")
        _ <- tx.put(hex"CAAB", hex"DEAD")
        } yield ()
        }
                                          rangeScan <- instance.read.use { tx =>
        tx.rangeScan(hex"AC", hex"C").toListL
        }
                                          } yield rangeScan
    }.runSyncUnsafe()
    assertEquals(res.toSet, Set((hex"ACAB", hex"DEAD"), (hex"ACAC", hex"BEEF"), (hex"ADD", hex"DDDD")))
}

#[test]
fn test_canceling_writes() {
    val res = createSlonky.instance.use { instance =>
                                          for {
                                          _ <- instance.write.use { tx =>
        for {
        _ <- tx.put(hex"ACAB", hex"DEAD")
        _ <- tx.put(hex"ACAC", hex"BEEF")
        } yield ()
        }
                                          _ <- instance.write.use { tx =>
        for {
        _ <- tx.put(hex"ADD", hex"DDDD")
        _ <- tx.put(hex"CAAB", hex"DEAD")
        _ <- tx.cancel()
        } yield ()
        }
                                          all <- instance.read.use { tx =>
        tx.scanAll().toListL
        }
                                          } yield all
    }.runSyncUnsafe()
    assertEquals(res.toSet, Set((hex"ACAB", hex"DEAD"), (hex"ACAC", hex"BEEF")))
}
