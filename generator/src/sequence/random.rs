use super::models::Sequence;
use crate::structs::range::Range;
use rand::Rng;

pub struct Random {
    pub seq1: Box<dyn Sequence<i64>>,
    pub seq2: Box<dyn Sequence<i64>>,
}

impl Random {
    pub fn new(seq1: Box<dyn Sequence<i64>>, seq2: Box<dyn Sequence<i64>>) -> Box<Random> {
        Box::new(Random { seq1, seq2 })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        let k1 = self.seq1.k_th(k);
        let k2 = self.seq2.k_th(k);
        let mut rng = rand::thread_rng();
        let a = f64::min(k1, k2);
        let b = f64::max(k1, k2);
        rng.gen_range(a..b)
    }
}

impl Sequence<f64> for Random {
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
