use std::cmp::Ordering;
use std::fmt;

// Can parameterize a datatype to make it able to contain a specified type of data.
struct Top<'a, T> {
    // Maximum number of elements to contain.
    count: usize,
    // Element ordering function.
    order: fn(&T, &T) -> Ordering,
    // Elements contained, by reference.
    elems: Vec<&'a T>,
}

// Cannot derive `Debug` for `Top`, because `order`.
impl<T: fmt::Debug> fmt::Debug for Top<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Top")
            .field("count", &self.count)
            .field("elems", &self.elems)
            .finish()
    }
}

// Can parameterize a function to make it able to take a specified type of data.
// Must parameterize an impl to match its type.
impl<'a, T> Top<'a, T> {
    fn new(count: usize, order: fn(&T, &T) -> Ordering, vals: &[T]) -> Top<'_, T> {
        let mut elems: Vec<&T> = vals.iter().collect();
        elems.sort_by(|&v1, &v2| order(v1, v2));
        if count < elems.len() {
            elems.truncate(count);
        }
        Top { count, order, elems }
    }

    fn update(&mut self, val: &'a T) {
        self.elems.push(val);
        let order = self.order;
        self.elems.sort_by(|&v1, &v2| order(v1, v2));
        if self.elems.len() > self.count {
            let _ = self.elems.pop();
        }
        assert!(self.elems.len() <= self.count);
    }
}

fn main() {
    let vals = ['d', 'b', 'c', 'a'];
    let mut top = Top::new(3, |x, y| x.cmp(y), &vals);
    println!("{:?}", top);
    Top::update(&mut top, &'X');
    println!("{:?}", top);
}
