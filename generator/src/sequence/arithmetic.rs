use crate::sequence::Sequence;

pub struct Arithmetic {
    pub start: f64,
    pub step: f64,
}

impl Arithmetic {
    pub fn new(start: f64, step: f64) -> Box<Arithmetic> {
        Box::new(Arithmetic { start, step })
    }
}

impl Sequence<f64> for Arithmetic {
    fn k_th(&self, k: usize) -> f64 {
        self.start + (k as f64) * self.step
    }
}