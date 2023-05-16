fn swap(a: &mut u8, b: &mut u8) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn main() {
    let mut a = [1u8, 2];
    let (a0, a1) = a.split_at_mut(1);
    swap(&mut a0[0], &mut a1[0]);
    println!("{:?}", a);
}
