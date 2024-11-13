use crate::sequence::Sequence;

pub struct LinearCombination {
    a: f64,
    b: f64,
    c: f64,
    seq1: Box<dyn Sequence<i64>>,
    seq2: Box<dyn Sequence<i64>>
}

impl LinearCombination {
    pub fn new( a: f64, b: f64, c: f64, seq1: Box<dyn Sequence<i64>>, seq2: Box<dyn Sequence<i64>>) -> Box<LinearCombination> {
        Box::new(LinearCombination { a, b, c, seq1, seq2 })
    }
}

impl Sequence<f64> for LinearCombination {
    fn k_th(&self, k: usize) -> f64 {
        let a_k = self.seq1.k_th(k); 
        let b_k = self.seq2.k_th(k); 
        self.a * a_k + self.b * b_k + self.c
    }
}