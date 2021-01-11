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

    let write_tx1 = slonky.write();

    tx.put(hex"ABAB", hex"CD");
    writeExists <- tx.keyExists(hex"ABAB");


    write_tx1.commit();
    let write_tx2 = slonky.write();
    tx.keyExists(hex"ABAB");

    write_tx2.commit();
    let read_tx = slonky.read();
    let read_exists1 = tx.keyExists(hex"ABAB");
    let read_exists2 =tx.key_exists(hex"AAAA");

     assert!(writeExists);
     assert!(writeExists2);
     assert!(readExists1);
     assert!(!read_exists2);
   }
 
   #[test]
   fn remove_a_pair() {
    let slonky = SlonkyInMemory::new();

    let write_tx1 = slonky.write();
    tx.put(hex"ABAB", hex"CD");
    tx.put(hex"AAAB", hex"DD");
    tx.remove(hex"AAAB");


    write_tx1.commit();

    let write_tx2 = slonky.write();
    //TODO check read
    tx.remove(hex"ABAB");
    //TODO check read
    tx.remove("ACCC");
    write_tx2.commit();



    let read_tx = slonky.read();
    tx.scanAll().compile.toList

    //TODO probably have more asserts
     assert!(res.isEmpty)
   }
 
   #[test]
   fn prefix_exists() {
    let slonky = SlonkyInMemory::new();

    let write_tx1 = slonky.write();
    tx.put(hex"ABAB", hex"CD");
    tx.put(hex"ABAC", hex"DC");
    tx.put(hex"CAAB", hex"DE");
    let write_exists1 = tx.prefixExists(hex"ABAC");


    write_tx1.commit();
    let write_tx2 = slonky.write();
    let write_exists2 = tx.prefixExists(hex"ABAB");
    write_tx2.commit();

    let read_tx = slonky.read();
    tx.prefixExists(hex"CA");

     assert!(writeExists);
     assert!(writeExists2);
     assert!(readExists);
   }
 
   #[test]
   fn pre_scan() {
    let slonky = SlonkyInMemory::new();

    let write_tx = slonky.write();
    write_tx.put(hex"ABAB", hex"CD");
    write_tx.put(hex"ABAC", hex"DC");
    write_tx.put(hex"CAAB", hex"DE");
    write_tx.commit();

    let read_tx = slonky.read();
    let prefix_scan = tx.prefixScan(hex"AB").compile.toList;

    assert_eq!(prefixScan.toSet, Set((hex"ABAB", hex"CD"), (hex"ABAC", hex"DC")));
   }
 
   #[test]
   fn ranges() {
    let slonky = SlonkyInMemory::new();

    let write_tx = slonky.write();
    write_tx.put(hex"AAAA", hex"AABB");
    write_tx.put(hex"ABAB", hex"BC");
    write_tx.put(hex"ABBB", hex"CD");
    write_tx.put(hex"ADDD", hex"DDDD");
    write_tx.put(hex"CAAB", hex"DEAD");
    write_tx.commit();

    let read_tx = slonky.read();
    let res = tx.rangeScan(hex"AB", hex"C0").collect();

     assert_eq!(res.toSet, Set((hex"ABAB", hex"BC"), (hex"ABBB", hex"CD"), (hex"ADDD", hex"DDDD")));
   }
 
   #[test]
   fn canceling_writes() {
    let slonky = SlonkyInMemory::new();

    let write_tx1 = slonky.write();
    _ <- tx.put(hex"AB", hex"CD");
    _ <- tx.put(hex"AC", hex"DC");
    write.commit();

    let write_tx2 = slonky.write();
    tx.put(hex"ADDD", hex"DDDD");
    tx.put(hex"CAAB", hex"DEAD");
    write_tx2.cancel();

    let read_tx = slonky.read();
    let res = tx.scanAll().collect();

     assert_eq!(res.toSet, Set((hex"AB", hex"CD"), (hex"AC", hex"DC")));
   }
}
