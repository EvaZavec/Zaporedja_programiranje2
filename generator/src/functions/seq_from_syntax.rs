use crate::sequence::Sequence;
use crate::sequence::arithmetic::Arithmetic;
use crate::sequence::constant::Constant;
use crate::sequence::chaos::Chaos;
use crate::sequence::cross_product::CrossProduct;
use crate::sequence::drop::Drop;
use crate::sequence::fibonacci::Fibonacci;
use crate::sequence::geometric::Geometric;
use crate::sequence::lin_comb::LinearCombination;
use crate::sequence::maximum::Maximum;
use crate::sequence::partial_product::PartialProduct;
use crate::sequence::product::Product;
use crate::sequence::quadratic::Quadratic;
use crate::sequence::random::Random;
use crate::sequence::sum::Sum;
use crate::sequence::switch::Switch;
use crate::sequence::from_elements::FromElements;
use crate::sequence::weighted_average::WeightedAverage;
use crate::structs::sequence::{SequenceRequest, SequenceSyntax};
use crate::structs::range::Range;

use super::get_other_sequences::pull_sequence;

pub async fn seq_from_syntax(syntax: &SequenceSyntax, range: &Range) -> Box<dyn Sequence<f64>> {
    match syntax.name.as_str() {
        "Arithmetic" => Arithmetic::new(syntax.parameters[0],syntax.parameters[1]),
        "Chaos" => Chaos::new(syntax.parameters[0],syntax.parameters[1]),
        "Constant" => Constant::new(syntax.parameters[0]),
        "CrossProduct" => CrossProduct::new(Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await,Box::pin(seq_from_syntax(&*syntax.sequences[1], range)).await),
        "Drop" => Drop::new(Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await,syntax.parameters[0]),
        "Fibonacci" => Fibonacci::new(Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await,
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range)).await, 
            Box::pin(seq_from_syntax(&*syntax.sequences[2], range)).await),
        "Geometric" => Geometric::new(syntax.parameters[0],syntax.parameters[1]),
        "LinComb" => LinearCombination::new(syntax.parameters[0],syntax.parameters[1],syntax.parameters[2],
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await,
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range)).await),
        "Maximum" => Maximum::new(Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await,
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range)).await),
        "PartialProduct" => PartialProduct::new(Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await),
        "Product" => Product::new(Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await,
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range)).await),
        "Quadratic" => Quadratic::new(syntax.parameters[0],syntax.parameters[1],syntax.parameters[2],
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await),
        "Random" => Random::new(Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await,
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range)).await),
        "Sum" => Sum::new(Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await,
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range)).await),
        "Switch" => Switch::new(syntax.parameters[0],syntax.parameters[1],syntax.parameters[2],
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await,
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range)).await),
        "WeightedAverage" => WeightedAverage::new(Box::pin(seq_from_syntax(&*syntax.sequences[0], range)).await,
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range)).await,
            Box::pin(seq_from_syntax(&*syntax.sequences[2], range)).await),
        _ => {
            let register_url = "http://0.0.0.0:7878/project";// ZAMENJAJ
            let result = pull_sequence(register_url, &syntax.name, SequenceRequest{
                range : range.clone(),
                parameters : syntax.parameters.clone(),
                sequences : syntax.sequences.clone(),
            }).await;

            let elem_vector = result.unwrap();
            
            FromElements::new(elem_vector)


        }
        
        //panic!("Sequence does not exist: {}", syntax.name),
    }
}