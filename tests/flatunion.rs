extern crate algorithms;

use algorithms::union_find::unionfind::*;
use algorithms::union_find::lazyflatunion::*;

#[test]
fn lazyflatunion_test() {
    let mut a: FlatUnion = FlatUnion::new(10);
    println!("Hello, world!");
    a.union(2, 3);
    a.union(3, 5);
    println!("Connected 2 and 3{:?}", a.connected(2, 3));
    assert!(a.connected(2, 3));
    assert!(!a.connected(3, 4));
    println!("Find by index 2: {:?}", a.find(2));
    assert_eq!(a.find(2), 5);
    assert_eq!(a.find(3), 5);
}