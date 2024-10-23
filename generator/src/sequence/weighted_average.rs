use super::models::Sequence;
use crate::structs::range::Range;

pub struct WeightedAverage {
    seq1: dyn Sequence<f64>,
    seq2: dyn Sequence<f64>,
    weight_seq: dyn Sequence<f64>
}

impl WeightedAverage {
    pub fn new(seq1: dyn Sequence<f64>, seq2: dyn Sequence<f64>, weight_seq: dyn Sequence) -> Box<WeightedAverage> {
        Box::new(WeightedAverage { seq1, seq2, weight_seq })
    }
    pub fn k_th(&self, k: usize) -> f64 {
        let ak = self.seq1.k_th(k);
        let bk = self.seq1.k_th(k);
        let wk = self.weight_seq.k_th(k);
        wk * ak + (1-wk) * bk       
    }
}

impl Sequence<f64> for WeightedAverage {
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