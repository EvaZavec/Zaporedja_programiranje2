use super::models::Sequence;
use crate::structs::range::Range;

pub struct Fibonacci {
    seq1: dyn Sequence<f64>,
    seq2: dyn Sequence<f64>,
    fib: dyn Sequence<f64>
}

impl Fibonacci {
    pub fn new(seq1: dyn Sequence<f64>, seq2: dyn Sequence<f64>, fib: dyn Sequence) -> Box<Fibonacci> {
        Box::new(Fibonacci { seq1, seq2, fib })
    }
    pub fn k_th(&self, k: usize) -> f64 {
        let ak = self.seq1.k_th(k);
        let bk = self.seq1.k_th(k);
        self.fib.k_th(k) * ak + self.fib.k_th(k-1) * bk       
    }
}

impl Sequence<f64> for Fibonacci {
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