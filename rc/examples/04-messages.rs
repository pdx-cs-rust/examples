use rc::message::Message;
use rc::misc::make_messages;
use rc::counter::Counter;

fn main() {
    let (m1, m2) = make_messages("m1", "m2");
    println!("{} {:?}", m1.text(), m1);
    println!("{} {:?}", m2.text(), m2);
    println!("{} {:?}", m1.text(), m1);
    println!("{} {:?}", m2.text(), m2);
}
