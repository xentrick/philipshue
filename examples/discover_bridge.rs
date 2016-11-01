extern crate philipshue;
use philipshue::bridge;

fn main() {
    let discovery = bridge::discover().unwrap().pop().unwrap();

    println!("Hue bridge found; IP: {}, ID: {}", discovery.ip(), discovery.id());
}
