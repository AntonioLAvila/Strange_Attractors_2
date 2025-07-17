
pub struct CircularBuffer<T, const N: usize> {
    buffer: [T; N],
    start: usize,
}

impl<T: Clone, const N: usize> CircularBuffer<T, N> {
    pub fn new(default: T) -> Self {
        Self {
            buffer: std::array::from_fn(|_| default.clone()),
            start: 0,
        }
    }

    pub fn push(&mut self, item: T) -> () {
        self.start = if self.start == 0 {N-1} else {self.start-1};
        self.buffer[self.start] = item;
    }

    pub fn get(&self, index: usize) -> &T {
        return &self.buffer[(self.start + index) % N];
    }
}


pub fn hsv_to_rgb(H: &f64, S: &f64, V: &f64) -> (f32, f32, f32) {
    let h = H % 1.0;
    let s = S.clamp(0.0, 1.0);
    let v = V.clamp(0.0, 1.0);

    let i = (h * 6.0).floor() as i32;
    let f = h * 6.0 - i as f64;

    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let (r, g, b) = match i.rem_euclid(6) {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => (0.0, 0.0, 0.0)
    };

    (r as f32, g as f32, b as f32)
}

