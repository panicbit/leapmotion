extern crate leap;

use leap::Controller;
use std::thread::sleep_ms;

fn main() {
    let controller = Controller::with_listener(DemoListener);
    loop {
        // Do something else
        sleep_ms(150);
    }
}

struct DemoListener;

impl leap::Listener for DemoListener {
    fn on_connect(&mut self, _: &Controller) {
        println!("Connected!");
    }

    fn on_frame(&mut self, controller: &Controller) {
        let frame = controller.frame();
        println!(
            "fps = {}, pointables = {}, fingers = {}",
            frame.current_fps(),
            frame.pointables().len(),
            frame.fingers().len(),
        );
        for pointable in frame.pointables().iter() {
            let stp = pointable.stabilized_tip_position();
            println!(
                "[p]: id = {id}, td = {td:.1}, stp = ({x:.1}, {y:.1}, {z:.1})",
                id = pointable.id(),
                td = pointable.touch_distance(),
                x = stp.x(),
                y = stp.y(),
                z = stp.z(),
            );
        }
        for finger in frame.fingers().iter() {
            let stp = finger.stabilized_tip_position();
            println!(
                "[f]: id = {id}, td = {td:.1}, stp = ({x:.1}, {y:.1}, {z:.1})",
                id = finger.id(),
                td = finger.touch_distance(),
                x = stp.x(),
                y = stp.y(),
                z = stp.z(),
            );
        }
        println!("--------------------------------");
        sleep_ms(150);
    }
}
