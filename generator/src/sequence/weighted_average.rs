use crate::sequence::Sequence;
use crate::structs::range::Range;

pub struct WeightedAverage {
    weight_seq: Box<dyn Sequence<i64>>,
    seq1: Box<dyn Sequence<i64>>,
    seq2: Box<dyn Sequence<i64>>
    
}

impl WeightedAverage {
    pub fn new(weight_seq: Box<dyn Sequence<i64>>, seq1: Box<dyn Sequence<i64>>, seq2: Box<dyn Sequence<i64>>) -> Box<WeightedAverage> {
        Box::new(WeightedAverage { weight_seq, seq1, seq2 })
    }
}

impl Sequence<f64> for WeightedAverage {
    fn k_th(&self, k: usize) -> f64 {
        let ak = self.seq1.k_th(k);
        let bk = self.seq1.k_th(k);
        let wk = self.weight_seq.k_th(k);
        wk * ak + (1.0-wk) * bk       
    }
    fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k < range.to {
            result.push(self.k_th(k as usize));
            k += range.step;
        }
        result
    }
}