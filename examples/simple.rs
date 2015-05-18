#[macro_use]
extern crate debugln;

fn main() {
    debugln!();
    debugln!("Hello");
    debugln!("Hello, {}", "world!");
}
