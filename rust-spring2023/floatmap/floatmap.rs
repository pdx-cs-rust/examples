use std::collections::{BTreeMap, HashMap};

use ordered_float::NotNan;

trait Map<K, V> {
    fn new() -> Self;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
}

impl<K: Eq + std::hash::Hash, V> Map<K, V> for HashMap<K, V> {
    fn new() -> Self {
        HashMap::new()
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        HashMap::insert(self, key, value)
    }
}

impl<K: Eq + Ord, V> Map<K, V> for BTreeMap<K, V> {
    fn new() -> Self {
        BTreeMap::new()
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        BTreeMap::insert(self, key, value)
    }
}

fn main() {
    fn demo<M: Map<NotNan<f32>, u32>>() -> M {
        let mut m = M::new();
        let kvs = [
            (0.1f32, 0u32),
            (0.2, 1),
        ];
        for (k, v) in kvs {
            m.insert(NotNan::new(k).unwrap(), v);
        }
        m
    }

    println!("{:?}", demo::<HashMap<_, _>>());
    println!("{:?}", demo::<BTreeMap<_, _>>());
}
