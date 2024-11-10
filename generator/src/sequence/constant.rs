use super::models::Sequence;
use crate::structs::range::Range;

pub struct Constant {
    pub constant: f64
}

impl Constant {
    pub fn new(constant: f64) -> Box<Constant> {
        Box::new(Constant { constant })
    }
}

impl Sequence<f64> for Constant {
    fn range(&self, range: &Range) -> Vec<f64> {
        let n = (range.to - range.from) / range.step;
        let result = vec![self.constant; n as usize];
        result
    }
}