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
fn make_top<T>(count: usize, order: fn(&T, &T) -> Ordering, vals: &[T]) -> Top<'_, T> {
    let mut elems: Vec<&T> = vals.iter().collect();
    elems.sort_by(|&v1, &v2| order(v1, v2));
    if count < elems.len() {
        elems.truncate(count);
    }
    Top { count, order, elems }
}

// Must parameterize an impl to match its type.
impl<'a, T> Top<'a, T> {
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
    let vals = [9, 7, 3, 1, 4, 6, 8, 2, 5];
    let mut top = make_top(3, |x, y| x.cmp(y), &vals);
    println!("{:?}", top);
    top.update(&0);
    println!("{:?}", top);
}
