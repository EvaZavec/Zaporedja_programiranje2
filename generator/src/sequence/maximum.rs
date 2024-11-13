use crate::sequence::Sequence;

pub struct Maximum {
    seq1: Box<dyn Sequence<i64>>,
    seq2: Box<dyn Sequence<i64>> 
}

impl Maximum {
    pub fn new(seq1: Box<dyn Sequence<i64>>, seq2: Box<dyn Sequence<i64>>) -> Box<Maximum> {
        Box::new(Maximum { seq1, seq2 })
    }
}

impl Sequence<f64> for Maximum {
    fn k_th(&self, k: usize)-> f64 {
        let a_k = self.seq1.k_th(k);
        let b_k = self.seq2.k_th(k);
        a_k.max(b_k)
    }
}