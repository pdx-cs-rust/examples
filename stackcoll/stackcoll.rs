pub struct Stack<T>(Vec<T>);

impl<T> std::iter::FromIterator<T> for Stack<T> {
    fn from_iter<I>(ii: I) -> Self
    where
        I: IntoIterator<Item = T>
    {
        let iter = ii.into_iter();
        let mut stack = Vec::with_capacity(iter.size_hint().1.unwrap_or(0));
        for v in iter {
            stack.push(v);
        }
        Stack(stack)
    }
}

#[test]
fn test_stack_collect() {
    let s: Stack<u8> = (0..7).collect();
    let v: Vec<u8> = (0..7).collect();
    assert_eq!(&s.0, &v);
}
