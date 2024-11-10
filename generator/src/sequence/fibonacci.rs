use crate::sequence::Sequence;
use crate::structs::range::Range;

pub struct Fibonacci {
    pub seq1: Box<dyn Sequence<i64>>,
    pub seq2: Box<dyn Sequence<i64>>,
    pub fib: Box<dyn Sequence<i64>>
}

impl Fibonacci {
    pub fn new(seq1: Box<dyn Sequence<i64>>, seq2: Box<dyn Sequence<i64>>, fib: Box<dyn Sequence<i64>>) -> Box<Fibonacci> {
        Box::new(Fibonacci { seq1, seq2, fib })
    }
<<<<<<< HEAD
}

impl Sequence<f64> for Fibonacci {
    fn k_th(&self, k: usize) -> f64 {
=======
    pub fn k_th(&self, k: usize) -> f64 {
        if k == 0 {
            return self.seq1.k_th(k) as f64;
        }
>>>>>>> 23d48fb5df05f7c1b9aeb651bcc640af8523f1d8
        let ak = self.seq1.k_th(k);
        let bk = self.seq2.k_th(k);
        self.fib.k_th(k) * ak + self.fib.k_th(k-1) * bk       
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