use super::models::Sequence;
use crate::structs::range::Range;

pub struct Constant {
    const: f64
}

impl Constant {
    pub fn new(const: f64) -> Box<Constant> {
        Box::new(Constant { const })
    }
}

impl Sequence<f64> for Constant {
    fn range(&self, range: &Range) -> Vec<f64> {
        let n = (range.to - range.from) / range.step;
        let result = vec![self.const; n];
        result
    }
}