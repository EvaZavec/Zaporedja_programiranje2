use crate::sequence::Sequence;

pub struct Geometric {
    pub start: f64,
    pub ratio: f64,
}

impl Geometric {
    pub fn new(start: f64, ratio: f64) -> Box<Geometric> {
        Box::new(Geometric { start, ratio })
    }
}

impl Sequence<f64> for Geometric {
    fn k_th(&self, k: usize) -> f64 {
        self.start * self.ratio.powf(k as f64)
    }
}