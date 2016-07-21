extern crate leap;

use std::thread::sleep_ms;
use leap::Controller;

fn main() {
    let controller = Controller::new();

    loop {
        if controller.is_connected() {
            let frame = controller.frame();
            println!("fps = {}, pointables = {}", frame.current_fps(), frame.pointables().count());
            for pointable in frame.pointables().iter() {
                println!("[p]: id = {}, td = {:.1}", pointable.id(), pointable.touch_distance());
            }
        }
        else {
            println!("Not connected!");
        }
        sleep_ms(150);
    }

}
