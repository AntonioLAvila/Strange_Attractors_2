
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

