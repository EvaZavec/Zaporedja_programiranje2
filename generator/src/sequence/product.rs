use super::models::Sequence;
use crate::structs::range::Range;

pub struct Product {
    pub seq1: Box<dyn Sequence<i64>>,
    pub seq2: Box<dyn Sequence<i64>>
}

impl Product {
    pub fn new(seq1: Box<dyn Sequence<i64>>, seq2: Box<dyn Sequence<i64>>) -> Box<Product> {
        Box::new(Product { seq1, seq2 })
    }
    pub fn k_th(&self, k: usize) -> f64 {
       self.seq1.k_th(k) * self.seq2.k_th(k)
    }
}

impl Sequence<f64> for Product {
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