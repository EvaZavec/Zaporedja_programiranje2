use crate::sequence::Sequence;
use crate::structs::range::Range;

pub struct Drop {
    pub seq: Box<dyn Sequence<i64>>,
    pub delay: f64
}

impl Drop {
    pub fn new(seq: Box<dyn Sequence<i64>>, delay: f64) -> Box<Drop> {
        Box::new(Drop { seq, delay })
    }
<<<<<<< HEAD
=======

    pub fn k_th(&self, k: usize) -> f64 {
        self.seq.k_th(k as f64 + self.delay)
    }
>>>>>>> 23d48fb5df05f7c1b9aeb651bcc640af8523f1d8
}

impl Sequence<f64> for Drop {
    fn k_th(&self, k: usize) -> f64 {
        self.seq.k_th((k as f64 + self.delay) as usize)
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