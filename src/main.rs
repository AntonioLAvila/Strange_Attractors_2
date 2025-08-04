extern crate kiss3d;

mod attractors;
mod util;

use kiss3d::nalgebra::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use attractors::*;

const dt: f32 = 0.01;

fn main() {
    let mut window = Window::new("Strange Attractors 2");
    window.set_light(Light::StickToCamera);
    window.set_line_width(5.);
    window.set_background_color(0.12156862745098039, 0.23921568627450981, 0.23921568627450981);

    let mut attractor = Attractor::<100, 100, Halvorsen>::new(Halvorsen::new());
    attractor.reset_points_random(-10., 10.);

    let color = Point3::new(0.592156862745098, 0.6941176470588235, 0.6509803921568628);

    while window.render() {
        attractor.update(dt);
        attractor.draw_solid(&mut window, Some(&color));
    }
}