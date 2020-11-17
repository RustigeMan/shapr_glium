extern crate shapr;
extern crate shapr_glium;

use shapr::units::*;
use shapr::Shp;
use shapr_glium::{ShaprGliumApp, AppConfig};

use std::time::Duration;

struct App {
    position: Pos,
    speed: Dlt<Pos>,
}

impl ShaprGliumApp for App {
    fn config(&self) -> AppConfig {
        AppConfig::default().title("Shapr-Glium Example: Rectangle")
    }

    fn draw_frame(&mut self, dt: Duration) -> Shp {
        let seconds = dt.as_millis() as f32 / 1000.0;
        self.position = self.position + self.speed * seconds;

        if self.position.0.abs() > 1.0 {
            self.speed.0.0 *= -1.0;
        }

        if self.position.1.abs() > 1.0 {
            self.speed.0.1 *= -1.0;
        }
        Shp::squa(0.5).trans(self.position).fill([1.0, 0.5, 0.0])
    }
}

fn main() {
    let app = App {
        position: Pos(0.0, 0.5),
        speed: Dlt(Pos(0.2, 0.2)),
    };

    shapr_glium::run_app(app);
}
