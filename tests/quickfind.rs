extern crate algorithms;

use algorithms::union_find::unionfind::*;
use algorithms::union_find::quickfind::*;

#[test]
fn quickfind_test() {
    let mut qf: QuickFind = QuickFind::new(5);
    println!("Hello, world!");
    qf.union(2, 3);
    println!("QF connected 2 and 3{:?}", qf.connected(2, 3));
    assert!(qf.connected(2, 3));
    assert!(!qf.connected(3, 4));
    println!("QF find by index 2: {:?}", qf.find(2));
    assert_eq!(qf.find(2), 3);
}