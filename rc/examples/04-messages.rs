use rc::misc::make_messages;

fn main() {
    let (m1, m2) = make_messages("m1", "m2");
    println!("{} {}", m1.text(), m1.accesses());
    println!("{} {}", m2.text(), m2.accesses());
    println!("{} {}", m1.text(), m1.accesses());
    println!("{} {}", m2.text(), m2.accesses());
}
