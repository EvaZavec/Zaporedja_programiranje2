use super::models::Sequence;
use crate::structs::range::Range;


pub struct Maximum {
    seq1: Box<dyn Sequence<f64>>,
    seq2: Box<dyn Sequence<f64>> 
}


impl Maximum {
    pub fn new(seq1: Box<dyn Sequence<f64>>, seq2: Box<dyn Sequence<f64>>) -> Box<Maximum> {
        Box::new(Maximum { seq1, seq2 })
    }
    pub fn k_th(&self, k: usize)-> f64 {
        let a_k = self.seq1.k_th(k);
        let b_k = self.seq2.k_th(k);
        a_k.max(b_k)
    }
}


impl Sequence<f64> for Maximum {
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