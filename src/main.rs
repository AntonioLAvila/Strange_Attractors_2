extern crate kiss3d;

mod attractors;
mod util;

use kiss3d::window::Window;
use kiss3d::light::Light;
use attractors::*;

const dt: f32 = 0.001;

fn main() {
    let mut window = Window::new("Strange Attractors 2");
    window.set_light(Light::StickToCamera);
    window.set_line_width(1.);

    let mut attractor = Attractor::<100, 100, Rossler>::new(Rossler::new());
    attractor.reset_points_random(-1., 1.);

    while window.render() {
        attractor.update(dt);
        attractor.draw(&mut window);
    }
}