pub mod arithmetic;
pub mod constant;
pub mod cross_product;
pub mod chaos;
pub mod drop;
pub mod fibonacci;
pub mod geometric;
pub mod partial_product;
pub mod product;
pub mod quadratic;
pub mod random;
pub mod sum;
pub mod switch;
pub mod weighted_average;
pub mod maximum;
pub mod lin_comb;
pub mod from_elements;
use crate::structs::range::Range;

pub trait Sequence<T> {
    fn k_th(&self, k: usize) -> T;//f64;

    fn range(&self, range: Range) -> Vec<T/*i64*/> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k < range.to {
            result.push(self.k_th(k as usize));
            k += range.step;
        }
        result
    }
}
