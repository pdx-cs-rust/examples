pub fn count<T, E>(values: T)
    where T: IntoIterator<Item=E>, E: std::fmt::Debug
{
    let mut iter = values.into_iter();
    while let Some(i) = iter.next() {
        println!("{i:?}");
    }
}

fn main() {
    count(1..=5);
    count('a'..='c');
    count(["hello", "world"]);
    let v = vec![(1, 2), (3, 4)];
    count(v.iter());
    count(v.iter());
}
