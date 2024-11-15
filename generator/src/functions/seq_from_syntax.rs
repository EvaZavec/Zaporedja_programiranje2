use crate::functions::http_handler::create_error_response;
use crate::sequence::{
    arithmetic::Arithmetic, 
    chaos::Chaos, 
    constant::Constant, 
    cross_product::CrossProduct, 
    drop::Drop, 
    fibonacci::Fibonacci, 
    from_elements::FromElements, 
    geometric::Geometric, 
    lin_comb::LinearCombination, 
    maximum::Maximum, 
    partial_product::PartialProduct, 
    product::Product, 
    quadratic::Quadratic, 
    random::Random, 
    sum::Sum, 
    switch::Switch, 
    weighted_average::WeightedAverage, 
    Sequence};
use crate::structs::range::Range;
use crate::structs::sequence::{SequenceRequest, SequenceSyntax};

use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::Response;

use super::get_other_sequences::pull_sequence;

pub async fn seq_from_syntax(
    syntax: &SequenceSyntax,
    range: &Range,
) -> Result<Box<dyn Sequence<f64>>, Response<BoxBody<Bytes, hyper::Error>>> {
    match syntax.name.as_str() {
        "Arithmetic" => Ok(Arithmetic::new(syntax.parameters[0], syntax.parameters[1])),
        "Chaos" => Ok(Chaos::new(syntax.parameters[0], syntax.parameters[1])),
        "Constant" => Ok(Constant::new(syntax.parameters[0])),
        "CrossProduct" => Ok(CrossProduct::new(
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range).await?),
        )),
        "Drop" => Ok(Drop::new(
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
            syntax.parameters[0],
        )),
        "Fibonacci" => Ok(Fibonacci::new(
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[2], range).await?),
        )),
        "Geometric" => Ok(Geometric::new(syntax.parameters[0], syntax.parameters[1])),
        "LinComb" => Ok(LinearCombination::new(
            syntax.parameters[0],
            syntax.parameters[1],
            syntax.parameters[2],
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range).await?),
        )),
        "Maximum" => Ok(Maximum::new(
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range).await?),
        )),
        "PartialProduct" => Ok(PartialProduct::new(
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
        )),
        "Product" => Ok(Product::new(
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range).await?),
        )),
        "Quadratic" => Ok(Quadratic::new(
            syntax.parameters[0],
            syntax.parameters[1],
            syntax.parameters[2],
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
        )),
        "Random" => Ok(Random::new(
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range).await?),
        )),
        "Sum" => Ok(Sum::new(
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range).await?),
        )),
        "Switch" => Ok(Switch::new(
            syntax.parameters[0],
            syntax.parameters[1],
            syntax.parameters[2],
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range).await?),
        )),
        "WeightedAverage" => Ok(WeightedAverage::new(
            Box::pin(seq_from_syntax(&*syntax.sequences[0], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[1], range).await?),
            Box::pin(seq_from_syntax(&*syntax.sequences[2], range).await?),
        )),
        _ => {
            let register_url = "http://0.0.0.0:7878/project"; //ZAMENJAJ
            match pull_sequence(
                register_url,
                &syntax.name,
                SequenceRequest {
                    range: range.clone(),
                    parameters: syntax.parameters.clone(),
                    sequences: syntax.sequences.clone(),
                },
            ).await
            {
                Ok(elem_vector) => Ok(FromElements::new(elem_vector)),
                Err(err_response) => {
                    let message = format!("Failed to fetch sequence: {}", syntax.name);
                    Err(create_error_response(hyper::StatusCode::BAD_REQUEST, message))
                }
            }
        }
    }
}
