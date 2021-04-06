fn main() {
    let mut x: [char; 2] = ['a', 'b'];
    println!("{:?} {}", x, x.len());

    let y = ['a', 'b'];
    assert_eq!(x, y);

    x = y;
    println!("{:?} {:?}", x, y);

    let z: &[char] = &x;
    assert_eq!(z, &y);
    println!("{:?} {}", z, z.len());

    let mut v = Vec::with_capacity(1);
    v.push('a');
    v.push('b');
    assert_eq!(v, x);

    let w = vec!['a', 'b'];
    assert_eq!(v, w);
}
