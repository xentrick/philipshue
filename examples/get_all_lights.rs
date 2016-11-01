extern crate philipshue;
use std::env;
use philipshue::bridge::{discover, Bridge};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return;
    }
    let bridge = Bridge::new(discover().unwrap().pop().unwrap().into_ip(), &*args[1]);

    match bridge.get_lights() {
        Ok(lights) => {
            println!("id name                 on    bri   hue sat temp");
            for ref l in lights.iter() {
                println!("{:2} {:20} {:5} {:3} {:5} {:3} {:4}K",
                         l.id,
                         l.light.name,
                         if l.light.state.on { "on" } else { "off" },
                         l.light.state.bri,
                         l.light.state.hue,
                         l.light.state.sat,
                         l.light.state.ct.map(|k| 1000000u32 / (k as u32)).unwrap_or(0));
            }
        }
        Err(err) => panic!(err),
    }
}
