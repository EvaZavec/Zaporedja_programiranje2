use super::models::Sequence;
use crate::structs::range::Range;

pub struct Sum {
    first: dyn Sequence<f64>,
    second: dyn Sequence<f64>
}

impl Sum {
    pub fn new(first: dyn Sequence<f64>, second: dyn Sequence<f64>) -> Box<Sum> {
        Box::new(Sum { first, second })
    }
    pub fn k_th(&self, k: usize) -> f64 {
       self.first::k_th(k) + self.second::k_th(k)
    }
}

impl Sequence<f64> for Sum {
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