use crate::sequence::Sequence;

pub struct PartialProduct {
    sequence: Box<dyn Sequence<i64>>,
}

impl PartialProduct {
    pub fn new(sequence: Box<dyn Sequence<i64>>) -> Box<PartialProduct> {
        Box::new(PartialProduct { sequence })
    }
}

impl Sequence<i64> for PartialProduct {
    fn name(&self) -> String {
        format!("Partial Product of {}", self.sequence.name())
    }

    fn contains(&self, item: i64) -> bool {
        let mut product = 1;
        let mut i = 0;
        while let Some(term) = self.sequence.k_th(i) {
            product *= term;
            if product == item {
                return true;
            }
            i += 1;
        }
        false
    }

    fn k_th(&self, k: i64) -> Option<i64> {
        let mut product = 1;
        for i in 0..=k {
            product *= self.sequence.k_th(i)?;
        }
        Some(product)
    }

    fn start(&self) -> Option<i64> {
        self.sequence.start()
    }
}