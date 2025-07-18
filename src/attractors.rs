use kiss3d::nalgebra::Point3;
use kiss3d::window::Window;
use rand::Rng;
use std::array::from_fn;

use crate::util::CircularBuffer;
use crate::util::hsv_to_rgb;

pub trait Dynamics {
    fn derivatives(&self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32);
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
        let mut color: Point3<f32> = Point3::new(0., 0., 0.);
        let step = (230. - 140.)/{L as f64}/255.;
        for i in 0..N {
            let buffer = &self.buffers[i];

            let mut h = 140./255.;
            let mut direction = 1.;
            for j in 0..L-2 {
                let (r, g, b) = hsv_to_rgb(&h, &1., &1.);
                color.x = r;
                color.y = g;
                color.z = b;

                window.draw_line(buffer.get(j), buffer.get(j+1), &color);

                if h <= 140./255. {
                    direction = 1.
                } else if h >= 230./255. {
                    direction = -1.
                }
                h += step*direction;
            }
        }
    }

    pub fn reset_points_random(&mut self, min: f32, max: f32) {
        let mut rng = rand::rng();

        self.points = from_fn(|_| {
            let x = rng.random_range(min..max);
            let y = rng.random_range(min..max);
            let z = rng.random_range(min..max);
            Point3::new(x, y, z)
        });

        self.buffers = self.points.map(|p| CircularBuffer::new(p));
    }
}



pub struct Halvorsen {
    a: f32
}
impl Dynamics for Halvorsen {
    fn derivatives(&self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
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

pub struct Lorentz {
    sigma: f32,
    rho: f32,
    beta: f32
}
impl Dynamics for Lorentz {
    fn derivatives(&self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        (
            (self.sigma*(y - x)) * dt,
            (x*(self.rho - z) - y) * dt,
            (x*y - self.beta*z) * dt,
        )
    }
}
impl Lorentz {
    pub fn new() -> Self {
        Self {
            sigma: 10.,
            rho: 28.,
            beta: 8./3.
        }
    }
}

pub struct Aizawa {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32
}
impl Dynamics for Aizawa {
    fn derivatives(&self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        (
            ((z - self.b)*x - self.d*y) * dt,
            (self.d*x + (z - self.b)*y) * dt,
            (self.c + self.a*z - z*z*z/3. - (x*x + y*y)*(1. - self.e*z) + self.f*z*x*x*x) * dt
        )
    }
}
impl Aizawa {
    pub fn new() -> Self {
        Self {
            a: 0.95,
            b: 0.7,
            c: 0.6,
            d: 3.5,
            e: 0.25,
            f: 0.1
        }
    }
}

pub struct FourWing {
    a: f32,
    b: f32,
    c: f32
}
impl Dynamics for FourWing {
    fn derivatives(&self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        (
            (self.a*x + y*z) * dt,
            (self.b*x + self.c*y - x*z) * dt,
            (-z - x*y) * dt
        )
    }
}
impl FourWing {
    pub fn new() -> Self {
        Self {
            a: 0.2,
            b: 0.01,
            c: -0.4
        }
    }
}

pub struct RabinovichFabrikant {
    alpha: f32,
    gamma: f32
}
impl Dynamics for RabinovichFabrikant {
    fn derivatives(&self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        (
            (y*(z - 1. + x*x) + self.gamma*x) * dt,
            (x*(3.*z + 1. - x*x) + self.gamma*y) * dt,
            (-2.*z*(self.alpha + x*y)) * dt
        )
    }
}
impl RabinovichFabrikant {
    pub fn new() -> Self {
        Self {
            alpha: 0.14,
            gamma: 0.1,
        }
    }
}

pub struct Thomas {
    b: f32,
}
impl Dynamics for Thomas {
    fn derivatives(&self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        (
            (y.sin() - self.b*x) * dt,
            (z.sin() - self.b*y) * dt,
            (x.sin() - self.b*z) * dt
        )
    }
}
impl Thomas {
    pub fn new() -> Self {
        Self {
            b: 0.208186,
        }
    }
}

pub struct ThreeScroll {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32
}
impl Dynamics for ThreeScroll {
    fn derivatives(&self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        (
            (self.a*(y - x) + self.d*x*z) * dt,
            (self.b*x - x*z + self.f*y) * dt,
            (self.c*z + x*y - self.e*x*x) * dt
        )
    }
}
impl ThreeScroll {
    pub fn new() -> Self {
        Self {
            a: 32.48,
            b: 45.84,
            c: 1.18,
            d: 0.13,
            e: 0.57,
            f: 14.7
        }
    }
}

pub struct Rossler {
    a: f32,
    b: f32,
    c: f32
}
impl Dynamics for Rossler {
    fn derivatives(&self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        (
            (-(y + z)) * dt,
            (x + self.a*y) * dt,
            (self.b + z*(x - self.c)) * dt,
        )
    }
}
impl Rossler {
    pub fn new() -> Self {
        Self {
            a: 0.2,
            b: 0.2,
            c: 5.7
        }
    }
}

pub struct Chen {
    alpha: f32,
    beta: f32,
    delta: f32
}
impl Dynamics for Chen {
    fn derivatives(&self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        (
            (self.alpha*x - y*z) * dt,
            (self.beta*y + x*z) * dt,
            (self.delta*z + x*y/3.) * dt,
        )
    }
}
impl Chen {
    pub fn new() -> Self {
        Self {
            alpha: 5.,
            beta: -10.,
            delta: -0.38
        }
    }
}