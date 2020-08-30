extern crate evdev;

use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::env;
use evdev::raw::eviocgrab;

// /dev/input/event7
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Requires two args: <input device> <second>")
    }

    let device_path = &args[1];
    let duration = Duration::from_secs(*&args[2].parse::<u64>().unwrap());
    let start = SystemTime::now();

    let mut touch_device = evdev::Device::open(device_path).unwrap();

    unsafe {
        let result = eviocgrab(touch_device.fd(), &1).unwrap();
        println!("ioctl returns: {}", result);
    }

    loop {
        for ev in &mut touch_device.events_no_sync().unwrap() {
            println!("{:?}", ev);
        }
        if start.elapsed().unwrap() > duration {
            break;
        }
        sleep(Duration::from_millis(50))
    }
}
