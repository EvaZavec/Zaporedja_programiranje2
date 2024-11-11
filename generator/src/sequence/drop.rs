use crate::sequence::Sequence;

pub struct Drop {
    pub seq: Box<dyn Sequence<i64>>,
    pub delay: f64
}

impl Drop {
    pub fn new(seq: Box<dyn Sequence<i64>>, delay: f64) -> Box<Drop> {
        Box::new(Drop { seq, delay })
    }
}

impl Sequence<f64> for Drop {
    fn k_th(&self, k: usize) -> f64 {
        self.seq.k_th((k as f64 + self.delay) as usize)
    }
}