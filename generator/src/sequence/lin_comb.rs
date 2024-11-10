use super::models::Sequence;
use crate::structs::range::Range;

pub struct LinearCombination {
    a: f64,
    b: f64,
    c: f64,
    seq1: Box<dyn Sequence<f64>>,
    seq2: Box<dyn Sequence<f64>>
}

impl LinearCombination {
    pub fn new( a: f64, b: f64, c: f64, seq1: Box<dyn Sequence<f64>>, seq2: Box<dyn Sequence<f64>>) -> Box<LinearCombination> {
        Box::new(LinearCombination { a, b, c, seq1, seq2 })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        let a_k = self.seq1.k_th(k); 
        let b_k = self.seq2.k_th(k); 
        self.a * a_k + self.b * b_k + self.c
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