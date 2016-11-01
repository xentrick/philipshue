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

    match bridge.get_all_lights() {
        Ok(lights) => {
            let max_name_len = lights.values().map(|l| l.name.len()).max().unwrap_or(4);
            println!("id {0:1$} on  bri hue   sat temp  alert   effect    colormode reachable xy", "name", max_name_len);
            for (id, light) in lights.iter() {
                println!("{:2} {:name_len$} {:3} {:3} {:5} {:3} {:4}K {:7} {:9} {:9} {:8} {:?}",
                         id,
                         light.name,
                         if light.state.on { "on" } else { "off" },
                         light.state.bri,
                         light.state.hue,
                         light.state.sat,
                         1000000u32 / (light.state.ct as u32),
                         light.state.alert,
                         light.state.effect,
                         if let Some(ref s) = light.state.colormode { s } else { "N/A" },
                         light.state.reachable,
                         light.state.xy,
                         name_len = max_name_len);
            }
        }
        Err(err) => panic!(err),
    }
}
