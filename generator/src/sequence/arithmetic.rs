use super::models::Sequence;
use crate::structs::range::Range;

pub struct Arithmetic {
    start: f64,
    step: f64,
}

impl Arithmetic {
    pub fn new(start: f64, step: f64) -> Box<Arithmetic> {
        Box::new(Arithmetic { start, step })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        self.start + (k as f64) * self.step
    }
}

impl Sequence<f64> for Arithmetic {
    fn range(&self, range: &Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut n = range.from;
        while n < range.to {
            result.push(self.k_th(n as usize));
            n += range.step;
        }
        result
    }
}