use geometria::core::GeoCore;
use geometria::script::Script;

fn main() {
    let mut core = GeoCore::init();
    core.add_script(BallContainer { balls: 69 });

    let _geo_window = core.create_window("hello 1", 1280, 720);

    core.run();
}

struct BallContainer {
    balls: u32,
}

impl Script for BallContainer {
    fn on_start(&mut self) {
        println!("I Start with {} balls!", self.balls);
    }

    fn on_update(&mut self) {
        self.balls += 1;
        println!("Now i have {} balls!", self.balls);
    }
}
