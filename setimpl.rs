use std::collections::HashMap;

pub type MyHashSet<T> = HashMap<T, ()>;

#[test]
fn test_my_hash_set() {
    let mut t: MyHashSet<u32> = HashMap::new();
    t.insert(3, ());
    t.insert(2, ());
    t.insert(3, ());
    let u: MyHashSet<u32> = vec![(2, ()), (3, ())].into_iter().collect();
    assert_eq!(t, u);
}
