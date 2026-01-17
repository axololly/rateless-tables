pub(crate) struct IndexGenerator {
    prng: u64,
    pub(crate) last_index: u64
}

impl IndexGenerator {
    pub fn new(prng: u64, last_index: u64) -> Self {
        Self { prng, last_index }
    }
}

impl Iterator for IndexGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.prng = self.prng.wrapping_mul(0xda942042e4dd58b5);

        let i = self.last_index as f64;
        let r = self.prng as f64;

        let factor = ((1_u64 << 32) as f64) / libm::sqrt(r + 1.0) - 1.0;
        let diff = libm::ceil((i + 1.5) * factor) as u64;

        self.last_index += diff;

        Some(self.last_index)
    }
}
