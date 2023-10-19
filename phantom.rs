use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
struct Hash<T> {
    h: u128,
    p: PhantomData<T>,
}

unsafe fn hasher(start: *const u8, nbytes: usize) -> u128 {
    let mut result: u128 = 0;
    for i in 0..nbytes {
        let b = *start.offset(i as isize);
        result = result.wrapping_add(b as u128);
    }
    result
}

impl<T> Hash<T> {
    fn hash(val: &T) -> Hash<T> {
        let h = unsafe { hasher(
            std::ptr::addr_of!(*val) as *const u8,
            std::mem::size_of::<T>(),
        )};
        Hash { h, p: PhantomData }
    }
}

fn main() {
    let h1 = Hash::hash(&(0i32, 1i32));
    println!("{:?}", h1);
    let h2 = Hash::hash(&1u64);
    println!("{:?}", h2);
    let h3 = Hash::hash(&(0i32, 1i32));
    println!("{:?}", h3);
    // Can't even compare these, since they are of
    // different type.
    // println!("{}", h1 == h2);
}
