fn main() {
    println!("{:02x}", 'x' as u8);
    println!("{:02x}", '🦀' as u8);
    println!("{:02x}", '🦀' as i8);
    println!("{:04x}", '🦀' as u16);
    println!("{:08x}", '🦀' as u32);
    println!("{:08x}", '🦀' as i32);
    println!("{:016x}", '🦀' as u64);

    println!("{}", 0x78u8 as char);
    // println!("{}", 0x1f980u32 as char); // Nope.
}
