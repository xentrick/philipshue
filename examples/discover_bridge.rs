ixtern crate philipshue;

mod discover;
use discover::discover;

fn main() {
    println!("Hue bridges found: {:#?}", discover());
}
