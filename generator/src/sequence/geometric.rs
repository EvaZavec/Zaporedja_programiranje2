use super::models::Sequence;
use crate::structs::range::Range;

pub struct Geometric {
    pub start: f64,
    pub ratio: f64,
}

impl Geometric {
    pub fn new(start: f64, ratio: f64) -> Box<Geometric> {
        Box::new(Geometric { start, ratio })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        self.start * self.ratio.powf(k as f64)
    }
}

impl Sequence<f64> for Geometric {
    fn range(&self, range: &Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k < range.to {
            result.push(self.k_th(k as usize));
            k += range.step;
        }
        result
    }
}