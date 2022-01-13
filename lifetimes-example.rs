struct Halves<'a, 'b, T> {
    first: &'a [T],
    last: &'b [T],
}

/*
fn half<'a, T>(source: &'a [T]) -> Halves<'a, 'a, T> {
    let halfway = source.len() / 2;
    Halves {
        first: &source[..halfway],
        last: &source[halfway..],
    }
}
*/

fn swap<'a, 'b, T>(source: Halves<'a, 'b, T>) -> Halves<'b, 'a, T> {
    Halves {
        first: source.last,
        last: source.first,
    }
}

/*
fn longer(s: String, t: String) -> String {
    if s.len() >= t.len() {
        s
    } else {
        t
    }
}
*/

fn main() {
    // let halves = half(&[1, 2, 3, 4, 5]);
    let first = vec![1, 2];
    let last = vec![3, 4, 5];
    let halves = Halves { first: &first, last: &last };
    let swapped = swap(halves);
    let new_first = swapped.first;
    drop(swapped);
    //drop(first);
    println!("{:?}", new_first);
}
