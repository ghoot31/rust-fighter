use gilrs::{EventType, Button};
use crate::common::observer::Observer;

pub struct ButtonPress;

impl Observer for ButtonPress {
    fn notify(&self, event: &EventType) {
        match event {
            EventType::ButtonPressed(button, _) => {
                let text = match button {
                    Button::DPadDown => { Some("↓") }
                    Button::DPadLeft => { Some("←") }
                    Button::DPadUp => { Some("↑") }
                    Button::DPadRight => { Some("→") }
                    Button::North => { Some("🟕") }
                    Button::South => { Some("Ⓧ") }
                    Button::East => { Some("Ⓞ") }
                    Button::West => { Some("🟗") }
                    _ => { None }
                };
                if text.is_some() {
                    println!("{}", text.unwrap());
                }
            }
            _ => {}
        }
    }
}
