use crate::sequence::Sequence;
use rand::Rng;

pub struct Switch {
    pub limit_up : f64,
    pub limit_down : f64,
    pub switch_index : f64,
    pub seq1: Box<dyn Sequence<f64>>,
    pub seq2: Box<dyn Sequence<f64>>,
}

impl Switch {
    pub fn new(limit_up : f64, limit_down : f64, switch_index : f64, seq1: Box<dyn Sequence<f64>>, seq2: Box<dyn Sequence<f64>>) -> Box<Switch> {
        Box::new(Switch { limit_up, limit_down, switch_index, seq1, seq2 })
    }
}

impl Sequence<f64> for Switch {
    fn k_th(&self, k: usize) -> f64 {
        let an = self.seq1.k_th(k);
        let bn = self.seq2.k_th(k);
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(self.limit_down..self.limit_down);
        if r < self.switch_index {an}
        else {bn}
    }
}