use crate::sequence::Sequence;
use crate::structs::range::Range;

pub struct Quadratic {
    pub a : f64,
    pub b : f64,
    pub c : f64,
    pub seq: Box<dyn Sequence<i64>>,
}

impl Quadratic {
    pub fn new(a : f64, b : f64, c : f64, seq: Box<dyn Sequence<i64>>,) -> Box<Quadratic> {
        Box::new(Quadratic { a, b, c, seq })
    }
    
}

impl Sequence<f64> for Quadratic {
    fn k_th(&self, k: usize) -> f64 {
        let xn = self.seq.k_th(k) as f64;
        self.a * xn.powf(k as f64) + self.b * xn + self.c
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