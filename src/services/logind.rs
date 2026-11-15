use amane::{Argument, Bus, Service, Value, arguments};
use std::{thread::spawn, time::Duration};

use crate::services::logind::Target::{Manager, Session};

pub struct Logind {
    system: Bus,
    session: Bus,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Target {
    Manager,
    Session,
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

fn session(method: &str, arguments: &[Argument]) -> Value {
    let login = Logind::read();
    login.system.call(
        "org.freedesktop.login1",
        "/org/freedesktop/login1/session/auto",
        "org.freedesktop.login1.Session",
        method,
        arguments,
    )
}

fn exec(target: Target, method: &str, arguments: &[Argument]) -> Value {
    match target {
        Manager => manager(method, arguments),
        Session => session(method, arguments),
    }
}

impl Logind {
    fn can(target: Target, operation: &str) -> bool {
        if exec(target, &format!("Can{}", operation), &[]).text() == "yes" {
            true
        } else {
            false
        }
    }

    fn no_check_operation(target: Target, operation: &'static str, timeout: u64, has_args: bool) {
        spawn(move || {
            std::thread::sleep(Duration::from_millis(timeout));
            let args = if has_args { arguments!(false) } else { vec![] };
            exec(target, operation, &args);
        });
    }

    fn operation(target: Target, operation: &'static str, timeout: u64) {
        let can = Logind::can(target, operation);

        if can {
            // TODO: NOTIFY
            Self::no_check_operation(target, operation, timeout, true);
        } else {
            // TODO: NOTIFY
        }
    }

    pub fn power_off() {
        Self::operation(Target::Manager, "PowerOff", 1000);
    }

    pub fn reboot() {
        Self::operation(Target::Manager, "Reboot", 1000);
    }

    pub fn suspend() {
        Self::operation(Target::Manager, "Suspend", 1000);
    }

    pub fn log_out() {
        Self::no_check_operation(Target::Session, "Terminate", 1000, false);
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
