use gilrs::EventType;

pub trait Observable {
    fn register(&mut self, observer: Box<dyn Observer>);
}

pub trait Observer {
    fn notify(&self, event: &EventType);
}
