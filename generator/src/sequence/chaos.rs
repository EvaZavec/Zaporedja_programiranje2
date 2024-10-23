use super::models::Sequence;
use crate::structs::range::Range;

pub struct Chaos {
    pub start: f64,
    pub chaos_parameter: f64
}

impl Chaos {
    pub fn new(start: f64, chaos_parameter: f64) -> Box<Chaos> {
        Box::new(Chaos {start, chaos_parameter })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        if k = 0 {self.start}
        else {
            let last = self.k_th(-1);
            self.chaos_parameter * last * (1 - last)
        }
    }
}

impl Sequence<f64> for Chaos {
    fn range(&self, range: &Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k < range.to {
            result.push(self.k_th(k as usize));
            k += range.step;
        }
        result
    }
}