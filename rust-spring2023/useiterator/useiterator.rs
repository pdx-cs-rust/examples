#[derive(Clone)]
struct MyArray([u64; 3]);

struct MyArrayIterator {
    a: MyArray,
    i: usize,
}

impl MyArrayIterator {
    fn new(a: MyArray) -> Self {
        Self { a, i: 0 }
    }
}

impl Iterator for MyArrayIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.a.0.len() {
            let result = self.a.0[self.i];
            self.i += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl IntoIterator for MyArray {
    type Item = u64;
    type IntoIter = MyArrayIterator;

    fn into_iter(self) -> Self::IntoIter {
        MyArrayIterator::new(self)
    }
}

fn main() {
    let a = MyArray([0, 1, 2]);
    for v in a.clone() {
    // for v in a.into_iter() {
    // let mut a_iter = a.into_iter();
    // while let Some(v) = a_iter.next() {
        print!("{} ", v);
    }
    println!();

    let mut a = [0, 1, 2];

    let va: Vec<_> = a.into_iter().collect();
    println!("{:?}", va);

    for v in &mut a {
    // for v in a.iter_mut() {
    // let mut a_iter = a.iter_mut();
    // while let Some(v) = a_iter.next() {
        *v += 1;
    }

    for v in &a {
    // for v in a.iter() {
    // let mut a_iter = a.iter();
    // while let Some(v) = a_iter.next() {
        print!("{} ", *v);
    }
    println!();
}
