fn main() {
    let c: char = '🦀';
    println!("{:x}", c as u32);
    println!("{}", char::from_u32(0x1f980_u32).unwrap());
    println!("\u{1f980}");

    println!("{}", "🦀".len());
    let crabby: Vec<u8> = "🦀".bytes().collect();
    println!("{:x?}", crabby);
    let crabby: Vec<char> = "xX🦀Xx".chars().collect();
    println!("{:x?}", crabby);
    let crabby_bytes = [0xf0_u8, 0x9f, 0xa6, 0x80];
    let crabby: &str = std::str::from_utf8(&crabby_bytes).unwrap();
    println!("{}", crabby);
    let crabby_owned: String = crabby.to_string();
    println!("{}", crabby_owned);

    let _ = std::fs::File::open("/etc/group").unwrap();

    use std::borrow::Cow;
    let cow: Cow<_> = Cow::Borrowed("🐄");
    println!("{}", cow);
    let cow: Cow<_> = Cow::Borrowed("🐄");
    println!("{}", cow);
    let cow: String = cow.into_owned();
    println!("{}", cow);
}
