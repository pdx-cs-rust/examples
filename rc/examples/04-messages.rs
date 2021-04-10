use rc::message::Message;
use rc::counter::Counter;

/// Make a couple of messages with given message strings.
pub fn make_messages(m1: &'static str, m2: &'static str)
                 -> (Message, Message)
{
    let counter = Counter::default();
    let m1 = Message::with_counter(m1, Counter::clone(&counter));
    let m2 = Message::with_counter(m2, counter);
    (m1, m2)
}

fn main() {
    let (m1, m2) = make_messages("m1", "m2");
    println!("{} {:?}", m1.text(), m1);
    println!("{} {:?}", m2.text(), m2);
    println!("{} {:?}", m1.text(), m1);
    println!("{} {:?}", m2.text(), m2);
}
