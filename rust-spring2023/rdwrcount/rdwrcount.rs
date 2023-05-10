use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
struct AccessCounter(usize);

impl AccessCounter {
    fn inc(&mut self) {
        self.0 += 1;
    }

    fn get(&self) -> usize {
        self.0
    }

    fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(Debug)]
struct Reader {
    counter: usize,
    accesses: Rc<RefCell<AccessCounter>>,
}

impl Reader {
    fn read(&mut self) -> String {
        self.accesses.borrow_mut().inc();
        self.counter += 1;
        self.counter.to_string()
    }

    fn get(&self) -> usize {
        self.counter
    }
}

#[derive(Debug)]
struct Writer {
    written: String,
    accesses: Rc<RefCell<AccessCounter>>,
}    

impl Writer {
    fn write(&mut self, s: &str) {
        self.accesses.borrow_mut().inc();
        self.written = s.to_owned();
    }

    fn get(&self) -> &str {
        &self.written
    }
}

fn main() {
    let accesses = Rc::new(RefCell::new(AccessCounter::default()));
    let mut writer = Writer {
        written: "".to_string(),
        accesses: Rc::clone(&accesses),
    };
    let mut reader = Reader {
        counter: 0,
        accesses: Rc::clone(&accesses),
    };
    writer.write(&reader.read());
    println!("{} {} {}", writer.get(), reader.get(), accesses.borrow().get());
    accesses.borrow_mut().reset();
    drop(accesses);
    println!("{} {}", writer.get(), reader.get());
}
