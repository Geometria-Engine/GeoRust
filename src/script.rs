pub trait Script {
    fn on_start(&mut self) {}
    fn on_update(&mut self) {}
}
