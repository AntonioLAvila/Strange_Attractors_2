extern crate kiss3d;

use kiss3d::window::Window;
use kiss3d::light::Light;

mod attractors;
mod circular_buffer;
use attractors::Halvorsen;

use crate::attractors::Attractor;

const dt: f32 = 0.01;

fn main() {
    let mut window = Window::new("Strange Attractors 2");
    window.set_light(Light::StickToCamera);
    window.set_line_width(1.);

    let mut attractor = Attractor::<100, 15, Halvorsen>::new(Halvorsen::new());

    while window.render() {
        attractor.update(dt);
        attractor.draw(&mut window);
    }
}