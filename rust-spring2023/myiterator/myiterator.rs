pub trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<T> MyIterator for Vec<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl MyIterator for std::ops::Range<u64> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let result = self.start;
            self.start += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[test]
fn test_my_iterator_vec() {
    let mut v = vec![1u64, 2, 3];
    assert_eq!(Some(3), v.next());
    assert_eq!(Some(2), v.next());
    assert_eq!(Some(1), v.next());
    assert_eq!(None, v.next());
    assert_eq!(None, v.next());
}

#[test]
fn test_my_iterator_range() {
    let mut r = 0..3;
    assert_eq!(Some(0), r.next());
    assert_eq!(Some(1), r.next());
    assert_eq!(Some(2), r.next());
    assert_eq!(None, r.next());
    assert_eq!(None, r.next());
}
