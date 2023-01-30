use geometria::Graphics::core::GeoCore;

fn main() {
    let mut core = GeoCore::init();

    let geo_window = core.create_window(1280, 720, "hello 1");

    core.run()
}
