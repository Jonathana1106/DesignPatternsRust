pub trait EventListener {
    fn on_event(&self, event: u32);
}

pub struct Notifier {
    listeners: Vec<Box<EventListener>>,
}

impl Notifier {
    pub fn new() -> Self {
        Self { listeners: Vec::new() }
    }

    pub fn register<T: EventListener + 'static>(&mut self, listener: T) {
        self.listeners.push(Box::new(listener));
    }

    pub fn notify(&self, event: u32) {
        for listener in &self.listeners {
            listener.on_event(event);
        }
    }
}

impl<F: Fn(u32)> EventListener for F {
    fn on_event(&self, event: u32) {
        self(event);
    }
}

fn main() {
    let mut notifier = Notifier::new();
    notifier.register(|event| println!("received [{}]", event));
    println!("notifying...");
    notifier.notify(42);
}

// pub struct Storage {
//     events: Vec<u32>,
// }
//
// impl Storage {
//     pub fn new() -> Self {
//         Self { events: Vec::new() }
//     }
//
//     pub fn store(&mut self, value: u32) {
//         self.events.push(value);
//     }
//
//     pub fn events(&self) -> &Vec<u32> {
//         &self.events
//     }
//
//     pub fn register_to(&self, notifier: &mut Notifier) {
//         notifier.register(move |event| {
//             /* how to retrieve a &mut Storage from here? */
//         });
//     }
//
// }
//
// fn main() {
//     use std::cell::RefCell;
//     use std::rc::Rc;
//
//     let mut notifier = Notifier::new();
//
//     // first Rc to the Storage
//     let rc = Rc::new(RefCell::new(Storage::new()));
//
//     // second Rc to the Storage
//     let rc2 = rc.clone();
//
//     // register the listener saving all the received events to the Storage
//     notifier.register(move |event| rc2.borrow_mut().store(event));
//
//     notifier.notify(3);
//     notifier.notify(141);
//     notifier.notify(59);
//     assert_eq!(&vec![3, 141, 59], rc.borrow().events());
//     println!("notifying...");
//     notifier.notify(42);
// }
