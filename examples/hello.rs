use geometria::graphics::core::GeoCore;
pub mod app;
use crate::app::appMain::AppMain;

#[allow(non_snake_case)]

fn main() {
    let mut core = GeoCore::init();

    let _geo_window = core.create_window("hello 1", 1280, 720);

    AppMain::Init();

    core.run();
}
