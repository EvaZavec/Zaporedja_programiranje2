use crate::sequence::Sequence;

pub struct Sum {
    seq1: Box<dyn Sequence<f64>>,
    seq2: Box<dyn Sequence<f64>>
}

impl Sum {
    pub fn new(seq1: Box<dyn Sequence<f64>>, seq2: Box<dyn Sequence<f64>>) -> Box<Sum> {
        Box::new(Sum { seq1, seq2 })
    }
}

impl Sequence<f64> for Sum {
    fn k_th(&self, k: usize) -> f64 {
        self.seq1.k_th(k) + self.seq2.k_th(k)
    }
}