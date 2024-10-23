use super::models::Sequence;
use crate::structs::range::Range;
use rand::Rng;

pub struct Random<S1, S2> {
    pub seq1: S1,
    pub seq2: S2,
}

impl<S1, S2> Random<S1, S2> {
    pub fn new(seq1: S1, seq2: S2) -> Box<Self> {
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

impl<S1, S2> Sequence<f64> for Random<S1, S2> {
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
