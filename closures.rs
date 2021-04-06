fn main() {
    let squaring = |n| n * n;
    assert_eq!(squaring(3), 9);

    let mut ss = 0;
    for n in 1..=3 {
        ss += n * n;
    }
    assert_eq!(14, ss);

    assert_eq!(14, (1..=3).map(squaring).sum());
}
