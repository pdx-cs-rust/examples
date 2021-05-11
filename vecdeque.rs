use std::collections::VecDeque;

fn main() {
    let mut q = VecDeque::new();
    q.push_back(1u32);
    q.push_back(2);
    println!("{}", q.pop_front().unwrap());
    println!("{}", q.pop_front().unwrap());
    assert_eq!(q.pop_front(), None);
}
