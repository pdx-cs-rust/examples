fn sum(numbers: &[u64]) -> u64 {
    let mut total = 0;
    for &n in numbers {
        total += n;
    }
    total
}

fn sum_fold(numbers: &[u64]) -> u64 {
    numbers.iter().fold(0, |a, &v| a + v)
}

fn main() {
    let a: [u64; 1000] = std::array::from_fn(|i| i as u64);
    println!("{}", sum(&a));
    println!("{}", a.iter().sum::<u64>());
    println!("{}", sum_fold(&a));

    let s: u64 = a
        .iter()
        .filter(|&v| v % 2 == 1)
        .map(|&v| v * v)
        .take(5)
        .sum();
    println!("{}", s);
    let mut t = 0;
    let mut ntaken = 0;
    for v in a {
        if ntaken >= 5 {
            break;
        }
        if v % 2 == 0 {
            continue;
        }
        t += v * v;
        ntaken += 1;
    }
    println!("{}", t);

    a.iter().take(5).for_each(|&v| println!("{}", v));
    for v in a.iter().take(5) {
        println!("{}", v);
    }
}
