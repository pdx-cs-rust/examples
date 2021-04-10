use std::cell::RefCell;
use std::rc::Rc;

use rc::message::Message;
use rc::misc::make_messages;

#[derive(Clone)]
struct MessageList {
    cur: Message,
    next: Option<Rc<RefCell<MessageList>>>,
}

impl MessageList {
    fn new(cur: Message) -> MessageList {
        MessageList { cur, next: None }
    }

    /// Print the message, then recursively print the
    /// attached message list.
    fn print(&self) {
        self.cur.print();
        if let Some(ref next) = self.next {
            print!(" -> ");
            next.borrow().print();
        }
    }

    /// Add a newline.
    fn println(&self) {
        self.print();
        println!();
    }

    /// Print this message and an indication of the next
    /// message if any.
    fn println_abbrev(&self) {
        self.cur.print();
        if let Some(ref next) = self.next {
            print!(" -> ");
            next.borrow().cur.print();
        }
        println!();
    }
}

/// Show simple use of messages with shared print counter.
fn message() {
    let (m1, m2) = make_messages("m1", "m2");
    m1.println();
    m2.println();
    m1.println();
    m2.println();
}

/// Make a couple of message lists with given message
/// strings and no tail.
fn make_message_lists(m1: &'static str, m2: &'static str)
                 -> (MessageList, MessageList)
{
    let (m1, m2) = make_messages(m1, m2);
    let ml1 = MessageList::new(m1);
    let ml2 = MessageList::new(m2);
    (ml1, ml2)
}

/// Show simple use of message lists.
fn message_list() {
    let (ml3, ml4) = make_message_lists("m3", "m4");
    ml3.println();
    ml4.println();
}

/// Make a cyclic message list.
fn message_list_cycle() {
    let (ml5, ml6) = make_message_lists("m5", "m6");
    let ml5 = Rc::new(RefCell::new(ml5));
    let ml6 = Rc::new(RefCell::new(ml6));
    ml5.borrow_mut().next = Some(ml6.clone());
    ml6.borrow_mut().next = Some(ml5.clone());
    ml5.borrow().println_abbrev();
    ml6.borrow().println_abbrev();
    //ml5.borrow().println();
    //ml6.borrow().println();
}

fn main() {
    message();
    println!();
    message_list();
    println!();
    message_list_cycle();
}
