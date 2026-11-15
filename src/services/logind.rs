use amane::{Argument, Bus, Service, Value, arguments};
use std::{thread::spawn, time::Duration};

pub struct Logind {
    system: Bus,
    session: Bus,
}

fn manager(method: &str, arguments: &[Argument]) -> Value {
    let login = Logind::read();
    login.system.call(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
        method,
        arguments,
    )
}

impl Logind {
    fn can(operation: &str) -> bool {
        if manager(&format!("Can{}", operation), &[]).text() == "yes" {
            true
        } else {
            false
        }
    }

    fn operation(operation: &'static str, timeout: u64) {
        let can = Logind::can(operation);

        // TODO: notify

        if can {
            spawn(move || {
                std::thread::sleep(Duration::from_millis(timeout));
                manager(operation, &arguments!(false));
            });
        } else {
            // TODO: notify
        }
    }

    pub fn power_off() {
        Self::operation("PowerOff", 1000);
    }
}

impl Service for Logind {
    fn new() -> Self {
        Self {
            system: Bus::system(),
            session: Bus::session(),
        }
    }

    // fn listen() {}
}
