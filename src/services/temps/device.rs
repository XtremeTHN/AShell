use glob::glob;

trait Device {
    fn name() -> String;
}

struct Cpu {}

struct Gpu {}
