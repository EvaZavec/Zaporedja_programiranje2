use crate::sequence::Sequence;

pub struct Constant {
    pub constant: f64
}

impl Constant {
    pub fn new(constant: f64) -> Box<Constant> {
        Box::new(Constant { constant })
    }
}

impl Sequence<f64> for Constant {
    fn k_th(&self, k: usize) -> f64 {
        self.constant
    }
}