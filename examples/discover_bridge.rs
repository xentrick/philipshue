extern crate philipshue;
use philipshue::bridge;
use philipshue::hue::Discovery;

fn main() {
    let discoveries = bridge::discover().unwrap();

    println!("Hue bridges found: {:#?}", discoveries);
}
