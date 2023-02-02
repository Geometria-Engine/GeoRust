use geometria::core::GeoCore;
use geometria::script::Script;

fn main() {
    let mut core = GeoCore::init();
    core.add_script(BallScript::new());

    let _geo_window = core.create_window("hello 1", 1280, 720);

    core.run();
}

struct BallScript {
    balls: u32,
}

impl BallScript {
    fn new() -> Self {
        Self { balls: 69 }
    }
}

impl Script for BallScript {
    fn on_start(&mut self) {
        println!("I Start with {} balls!", self.balls);
    }

    fn on_update(&mut self) {
        self.balls += 1;
        println!("Now i have {} balls!", self.balls);
    }
}
