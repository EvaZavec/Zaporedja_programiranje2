use crate::sequence::Sequence;

pub struct Chaos {
    pub start: f64,
    pub chaos_parameter: f64
}

impl Chaos {
    pub fn new(start: f64, chaos_parameter: f64) -> Box<Chaos> {
        Box::new(Chaos { start, chaos_parameter })
    }
}

impl Sequence<f64> for Chaos {
    fn k_th(&self, k: usize) -> f64 {
        if k == 0 {self.start}
        else {
            let last = self.k_th(k-1);
            self.chaos_parameter * last * (1.0 - last)
        }
    }
}
