use geometria::graphics::core::GeoCore;

fn main() {
    let mut core = GeoCore::init();

    let _geo_window = core.create_window("hello 1", 1280, 720);

    core.run()
}
