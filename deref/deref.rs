pub struct MyStruct(Box<u8>);

impl std::ops::Deref for MyStruct {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

#[test]
fn test_deref() {
    fn add_one(x: &u8) -> u8 {
        *x + 1
    }
    let s = MyStruct(Box::new(8));
    assert_eq!(&*s, &8u8);
    assert_eq!(add_one(&s), 9u8);
}
