use super::models::Sequence;
use crate::structs::range::Range;

pub struct Drop {
    pub seq: dyn Sequence<f64>,
    pub delay: f64
}

impl Drop {
    pub fn new(seq: Sequence, delay: f64) -> Box<Drop> {
        Box::new(Drop { seq, delay })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        self.seq.k_th(k + self.delay)
    }
}

impl Sequence<f64> for Drop {
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