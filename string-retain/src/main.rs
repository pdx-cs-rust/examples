fn retain<F>(s: &mut String, mut pred: F)
    where F: FnMut(char) -> bool
{
    let mut nretained = 0;
    for c in s.chars() {
        if pred(c) {
            nretained += 1;
        }
    }
    let mut result = String::with_capacity(nretained);
    for c in s.chars() {
        if pred(c) {
            result.push(c);
        }
    }
    *s = result;
}

fn main() {
    let mut s = "Hello, world!".to_string();
    let mut copy_of_s = s.clone();
    let is_vowel = |c| "aieou".contains(c);
    copy_of_s.retain(is_vowel);
    println!("{}", copy_of_s);
    retain(&mut s, is_vowel);
    println!("{}", s);
}
