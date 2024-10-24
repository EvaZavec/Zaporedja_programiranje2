use super::models::Sequence;
use crate::structs::range::range;

pub struct LinearCombination {
    seq1: Box<dyn Sequence<f64>>,
    seq2: Box<dyn Sequence<f64>>,
    x: f64,
    y: f64
}

impl LinearCombination {
    pub fn new(seq1: Box<dyn Sequence<f64>>, seq2: Box<dyn Sequence<f64>>, x: f64, y: f64, z: f64) -> Box<LinearCombination> {
        Box::new(LinearCombination { seq1, seq2, x, y })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        let a_k = self.seq1.k_th(k); 
        let b_k = self.seq2.k_th(k); 
        self.x * a_k + self.y * b_k
    }
}

impl Sequence<f64> for LinearCombination {
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