use crate::common::observer::Observer;
use gilrs::{EventType, Button};

pub struct StartPress;

impl Observer for StartPress {
    fn notify(&self, event: &EventType) {
        match event {
            EventType::ButtonPressed(button, _) => {
                match button {
                    Button::Start => {
                        println!("Bye! :)");
                        std::process::exit(0);
                    }
                    _ => { }
                };
            }
            _ => {}
        }
    }
}
