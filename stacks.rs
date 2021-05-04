use std::collections::LinkedList;

pub trait Stack<T> {
    fn spush(&mut self, v: T);
    fn spop(&mut self) -> Option<T>;
}

impl<'a, T: ?Sized> Stack<&'a T> for Vec<&'a T> {
    fn spush(&mut self, v: &'a T) {
        self.push(v);
    }

    fn spop(&mut self) -> Option<&'a T> {
        self.pop()
    }
}

impl<'a, T: ?Sized> Stack<&'a T> for LinkedList<&'a T> {
    fn spush(&mut self, v: &'a T) {
        self.push_front(v);
    }

    fn spop(&mut self) -> Option<&'a T> {
        self.pop_front()
    }
}

pub fn dup<'a, 'b: 'a, S, T>(stack: &mut S)
    where S: Stack<&'b T> + 'a, T: 'b + ?Sized
{
    let v = stack.spop().unwrap();
    stack.spush(v);
    stack.spush(v);
}

#[cfg(test)]
mod test {
    use crate::*;

    fn test<S>(mut s: S) where S: Stack<&'static str> {
        s.spush("hello");
        dup(&mut s);
        assert_eq!(s.spop().unwrap(), "hello");
        assert_eq!(s.spop().unwrap(), "hello");
        assert_eq!(s.spop(), None);
    }

    #[test]
    fn test_impls() {
        test(Vec::new());
        test(LinkedList::new());
    }
}
