use kiss3d::nalgebra::Point3;
use kiss3d::window::Window;
use rand::Rng;
use std::array::from_fn;

use crate::circular_buffer::CircularBuffer;

pub trait Dynamics {
    fn derivatives(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32);
}

pub struct Attractor<const N: usize, const L: usize, D: Dynamics>{
    buffers: [CircularBuffer<Point3<f32>, L>; N],
    points: [Point3<f32>; N],
    dynamics: D
}
impl<const N: usize, const L: usize, D: Dynamics> Attractor<N, L, D> {

    pub fn new(dynamics: D) -> Self {
        let mut rng = rand::rng();

        let points = from_fn(|_| {
            let x = rng.random_range(-20.0..20.0);
            let y = rng.random_range(-20.0..20.0);
            let z = rng.random_range(-20.0..20.0);
            Point3::new(x, y, z)
        });

        let buffers = points.map(|p| CircularBuffer::new(p));

        Self {
            buffers,
            points,
            dynamics,
        }
    }

    pub fn update(&mut self, dt: f32) {
        for i in 0..N {
            let p = &mut self.points[i];
            let (dx, dy, dz) = self.dynamics.derivatives(p.x, p.y, p.z, dt);

            p.x += dx;
            p.y += dy;
            p.z += dz;

            self.buffers[i].push(p.clone());
        }
    }

    pub fn draw(&self, window: &mut Window) {
        let color: Point3<f32> = Point3::new(0., 1., 0.);
        for i in 0..N {
            let buffer = &self.buffers[i];
            for j in 0..L-2 {
                window.draw_line(buffer.get(j), buffer.get(j+1), &color);
            }
        }
    }
}



pub struct Halvorsen {
    a: f32
}
impl Dynamics for Halvorsen {
    fn derivatives(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        (
            (-self.a*x - 4.*y - 4.*z -y*y) * dt,
            (-self.a*y - 4.*z - 4.*x -z*z) * dt,
            (-self.a*z - 4.*x - 4.*y -x*x) * dt
        )
    }
}
impl Halvorsen {
    pub fn new() -> Self {
        Self {a: 1.89}
    }
}