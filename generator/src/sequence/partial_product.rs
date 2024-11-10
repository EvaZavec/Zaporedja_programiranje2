use crate::sequence::Sequence;
use crate::structs::range::Range;

pub struct PartialProduct {
    pub seq: Box<dyn Sequence<i64>>,
}

impl PartialProduct {
    pub fn new(seq: Box<dyn Sequence<i64>>) -> Box<PartialProduct> {
        Box::new(PartialProduct { seq })
    }
}

impl Sequence<i64> for PartialProduct {

    fn k_th(&self, k: usize) -> f64 {
        let mut product = 1.0;
        for i in 0..=k {
            let p = self.seq.k_th(i);
            product *= p;
        }
        product
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
