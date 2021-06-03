fn main() {
    let mut p: * const u32 = std::ptr::null::<u32>();
    println!("{:?}", p);
    println!("{:?}", p.is_null());
    p = p.wrapping_add(1);
    println!("{:?}", p);

    let p: *const u8 = "🦀ello".as_ptr();
    println!("{}", unsafe { (*p) as char });
}
