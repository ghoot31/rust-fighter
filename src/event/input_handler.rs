use gilrs::{Gilrs, Event};
use crate::common::observer::{Observer, Observable};

pub struct InputHandler {
    gilrs: Gilrs,
    observers: Vec<Box<dyn Observer>>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler { gilrs: Gilrs::new().unwrap(), observers: Vec::new() }
    }

    pub fn load_controllers(&mut self) {
        for (_id, gamepad) in self.gilrs.gamepads() {
            println!("{} detected!", gamepad.name());
        }
    }

    pub fn handle(&mut self) {
        while let Some(Event { id: _, event, time: _ }) = self.gilrs.next_event() {
            for observer in &self.observers {
                observer.notify(&event);
            }
        }
    }
}

impl Observable for InputHandler {
    fn register(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }
}
