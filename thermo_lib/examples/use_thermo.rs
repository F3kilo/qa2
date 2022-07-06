use std::{thread, time::Duration};

use thermo_lib::SmartThermo;

fn main() {
    let receiver_address = "127.0.0.1:4321";
    let thermo = SmartThermo::new(receiver_address).unwrap();
    for _ in 0..120 {
        thread::sleep(Duration::from_secs(1));
        let temperature = thermo.get_temperature();
        println!("The temperature is {temperature}");
    }
}