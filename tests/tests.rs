// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use slonky_in_memory::SlonkyInMemory;

   #[test]
   fn instances_should_start_empty() {
     let slonky = SlonkyInMemory::new();
     let read_tx = slonky.read();
     let res = read_tx.scanAll().collect();
     assert!(res.isEmpty)
   }

   #[test]
   fn add_a_single_pair() {
    let slonky = SlonkyInMemory::new();
    let write_tx = slonky.write();
    write_tx.put(hex"ABAB", hex"CD");
    write_tx.commit();
    let read_tx = slonky.read();
    let res = tx.scan_all().compile.toList.toSet
    assert_eq!(res, Set((hex"ABAB", hex"CD")))
   }
 
   #[test]
   fn access_a_pair_during_and_after_a_write() {
    let slonky = SlonkyInMemory::new();
    let write_tx1 = slonky.write();
    write_tx1.put(hex"ABAB", hex"CD");
    let write_get1 = write_tx.get(hex"ABAB");
    write_tx1.commit();

    let write_tx2 = slonky.write();
    tx.get(hex"ABAB")

    write_tx1.cancel();

    let read_tx = slonky.read();
    let read_get = read_tx.get(hex"ABAB");

     assert_eq!(write_get1, Some(hex"CD"))
     assert_eq!(write_get2, Some(hex"CD"))
     assert_eq!(read_get, Some(hex"CD"))
   }
 
   #[test]
   fn check_if_a_key_exists_during_and_after_a_write() {
    let slonky = SlonkyInMemory::new();

     let  (writeExists, writeExists2, readExists) = createSlonky.instance.use { instance =>
       for {
         writeExists <- instance.write.use { tx =>
           for {
             _           <- tx.put(hex"ABAB", hex"CD")
             writeExists <- tx.keyExists(hex"ABAB")
           } yield writeExists
         }
         writeExists2 <- instance.write.use { tx =>
           tx.keyExists(hex"ABAB")
         }
         readExists <- instance.read.use { tx =>
           tx.keyExists(hex"ABAB")
         }
       } yield (writeExists, writeExists2, readExists)
     }.unsafeRunSync()
     assert!(writeExists)
     assert!(writeExists2)
     assert!(readExists)
   }
 
   #[test]
   fn remove_a_pair() {
    let slonky = SlonkyInMemory::new();

     let  res = createSlonky.instance.use { instance =>
       for {
         _ <- instance.write.use { tx =>
           tx.put(hex"ABAB", hex"CD")
         }
         _ <- instance.write.use { tx =>
           tx.remove(hex"ABAB")
         }
         all <- instance.read.use { tx =>
           tx.scanAll().compile.toList
         }
       } yield all
     }.unsafeRunSync().toSet
     assert!(res.isEmpty)
   }
 
   #[test]
   fn prefix_exists() {
    let slonky = SlonkyInMemory::new();

     let  (writeExists, writeExists2, readExists) = createSlonky.instance.use { instance =>
       for {
         writeExists <- instance.write.use { tx =>
           for {
             _ <- tx.put(hex"ABAB", hex"CD")
             _ <- tx.put(hex"ABAC", hex"DC")
             _ <- tx.put(hex"CAAB", hex"DE")
             writeExists <- tx.prefixExists(hex"ABAC")
           } yield writeExists
         }
         writeExists2 <- instance.write.use { tx =>
           tx.prefixExists(hex"ABAB")
         }
         readExists <- instance.read.use { tx =>
           tx.prefixExists(hex"CA")
         }
       } yield (writeExists, writeExists2, readExists)
     }.unsafeRunSync()
     assert!(writeExists)
     assert!(writeExists2)
     assert!(readExists)
   }
 
   #[test]
   fn pre_scan() {
    let slonky = SlonkyInMemory::new();

     let  prefixScan = createSlonky.instance.use { instance =>
       for {
         _ <- instance.write.use { tx =>
           for {
             _ <- tx.put(hex"ABAB", hex"CD")
             _ <- tx.put(hex"ABAC", hex"DC")
             _ <- tx.put(hex"CAAB", hex"DE")
           } yield ()
         }
         prefixScan <- instance.read.use { tx =>
           tx.prefixScan(hex"AB").compile.toList
         }
       } yield (prefixScan)
     }.unsafeRunSync()
     assert_eq!(prefixScan.toSet, Set((hex"ABAB", hex"CD"), (hex"ABAC", hex"DC")))
   }
 
   #[test]
   fn ranges() {
    let slonky = SlonkyInMemory::new();

     let  res = createSlonky.instance.use { instance =>
       for {
         _ <- instance.write.use { tx =>
           for {
             _ <- tx.put(hex"AAAA", hex"AABB")
             _ <- tx.put(hex"ABAB", hex"BC")
             _ <- tx.put(hex"ABBB", hex"CD")
             _ <- tx.put(hex"ADDD", hex"DDDD")
             _ <- tx.put(hex"CAAB", hex"DEAD")
           } yield ()
         }
         rangeScan <- instance.read.use { tx =>
           tx.rangeScan(hex"AB", hex"C0").compile.toList
         }
       } yield rangeScan
     }.unsafeRunSync()
     assert_eq!(res.toSet, Set((hex"ABAB", hex"BC"), (hex"ABBB", hex"CD"), (hex"ADDD", hex"DDDD")))
   }
 
   #[test]
   fn canceling_writes() {
    let slonky = SlonkyInMemory::new();

     let  res = createSlonky.instance.use { instance =>
       for {
         _ <- instance.write.use { tx =>
           for {
             _ <- tx.put(hex"AB", hex"CD")
             _ <- tx.put(hex"AC", hex"DC")
           } yield ()
         }
         _ <- instance.write.use { tx =>
           for {
             _ <- tx.put(hex"ADDD", hex"DDDD")
             _ <- tx.put(hex"CAAB", hex"DEAD")
             _ <- tx.cancel()
           } yield ()
         }
         all <- instance.read.use { tx =>
           tx.scanAll().compile.toList
         }
       } yield all
     }.unsafeRunSync()
     assert_eq!(res.toSet, Set((hex"AB", hex"CD"), (hex"AC", hex"DC")))
   }
}
