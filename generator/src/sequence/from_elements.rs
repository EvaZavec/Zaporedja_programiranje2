use crate::sequence::Sequence;
use crate::structs::range::Range;

pub struct FromElements {
    pub elems: Vec<f64> 
}

impl FromElements {
    pub fn new(elems: Vec<f64>) -> Box<FromElements> {
        Box::new(FromElements { elems })
    }
}

impl Sequence<f64> for FromElements {
    fn k_th(&self, k: usize) -> f64 {
        if (k>=self.elems.len()) {
            return self.elems[0]
        }
        return self.elems[k]
    }
    fn range(&self, range: Range) -> Vec<f64> {
        return self.elems.clone()
    }
}