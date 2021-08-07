use std::time::{Duration, SystemTime};
use std::thread::sleep;
use event::input_handler::InputHandler;
use crate::common::observer::Observable;
use crate::model::listener;

mod common;
mod event;
mod model;

const FPS_CAP: i8 = 60;
const FRAME_DURATION_MICROSECONDS: u128 = 1000000 / FPS_CAP as u128;

fn main() {
    let mut input_handler = InputHandler::new();
    input_handler.load_controllers();
    register_listeners(&mut input_handler);
    loop {
        let start_time = SystemTime::now();
        // model updates using observer pattern
        input_handler.handle();
        // TODO: implement view layer
        sleep(get_sleep_duration(start_time));
    }
}

fn register_listeners(input_handler: &mut InputHandler) {
    input_handler.register(Box::new(listener::button_press::ButtonPress));
    input_handler.register(Box::new(listener::start_press::StartPress))
}

fn get_sleep_duration(start_time: SystemTime) -> Duration {
    let end_time = SystemTime::now();
    let execution_time = match end_time.duration_since(start_time) {
        Ok(duration) => { duration }
        Err(_) => {
            // TODO: move to dedicated error Handler static call
            eprint!("Error ocurred when computing duration");
            std::process::exit(2)
        }
    };

    Duration::from_micros((FRAME_DURATION_MICROSECONDS - execution_time.as_micros()) as u64)
}
