use crate::sequence::Sequence;
use rand::Rng;

pub struct Random {
    pub seq1: Box<dyn Sequence<i64>>,
    pub seq2: Box<dyn Sequence<i64>>,
}

impl Random {
    pub fn new(seq1: Box<dyn Sequence<i64>>, seq2: Box<dyn Sequence<i64>>) -> Box<Random> {
        Box::new(Random { seq1, seq2 })
    }
}

impl Sequence<f64> for Random {
    fn k_th(&self, k: usize) -> f64 {
        let k1 = self.seq1.k_th(k) as f64;
        let k2 = self.seq2.k_th(k) as f64;
        let mut rng = rand::thread_rng();
        let a = f64::min(k1, k2);
        let b = f64::max(k1, k2);
        rng.gen_range(a..b)
    }
}
