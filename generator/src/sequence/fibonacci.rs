use crate::sequence::Sequence;

pub struct Fibonacci {
    pub seq1: Box<dyn Sequence<i64>>,
    pub seq2: Box<dyn Sequence<i64>>
}

impl Fibonacci {
    pub fn new(seq1: Box<dyn Sequence<i64>>, seq2: Box<dyn Sequence<i64>>) -> Box<Fibonacci> {
        Box::new(Fibonacci { seq1, seq2 })
    }
}
impl Sequence<f64> for Fibonacci {
    fn k_th(&self, k: usize) -> f64 {
        if k == 0 {
            return self.seq1.k_th(k) as f64;
        }
        let ak = self.seq1.k_th(k);
        let bk = self.seq2.k_th(k);
        self.k_th(k-2) * ak + self.k_th(k-1) * bk       
    }
}