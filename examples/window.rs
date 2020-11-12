extern crate shapr;
extern crate shapr_glium;

use shapr::Shp;
use shapr_glium::{AppConfig, ShaprGliumApp};

use std::time::Duration;

struct App {}

impl ShaprGliumApp for App {
    fn config(&self) -> AppConfig {
        AppConfig::new_with_title("Shapr-Glium Example: Window")
    }

    fn draw_frame(&mut self, _dt: Duration) -> Shp {
        Shp::nil()
    }
}

fn main() {
    let app = App {};

    shapr_glium::run_app(app);
}
