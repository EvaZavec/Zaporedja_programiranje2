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
use crate::sequence::weighted_average::WeightedAverage;

use crate::structs::sequence::SequenceSyntax;
use crate::info::sequences;
use crate::structs::sequence::SequenceRequest;
use crate::functions::project_handler::get_project;
use crate::functions::http_handler::{full, collect_body, create_404};

use bytes::Bytes;
use hyper::{body::Incoming, Method, Request, Response};
use http_body_util::combinators::BoxBody;

fn to_syntax(name: &String, parameters: Vec<f64>, sequences: Vec<Box<SequenceSyntax>>) -> SequenceSyntax {
    SequenceSyntax {
        name : name.clone(),
        parameters,
        sequences,
    }
}

pub fn seq_from_syntax(syntax: &SequenceSyntax) -> Box<dyn Sequence<f64>> {
    match syntax.name.as_str() {
        "Arithmetic" => Arithmetic::new(syntax.parameters[0],syntax.parameters[1]),
        "Chaos" => Chaos::new(syntax.parameters[0],syntax.parameters[1]),
        "Constant" => Constant::new(syntax.parameters[0]),
        "CrossProduct" => CrossProduct::new(seq_from_syntax(&*syntax.sequences[0]),seq_from_syntax(&*syntax.sequences[1])),
        "Drop" => Drop::new(seq_from_syntax(&*syntax.sequences[0]),syntax.parameters[0]),
        "Fibonacci" => Fibonacci::new(seq_from_syntax(&*syntax.sequences[0]),
            seq_from_syntax(&*syntax.sequences[1]), 
            seq_from_syntax(&*syntax.sequences[2])),
        "Geometric" => Geometric::new(syntax.parameters[0],syntax.parameters[1]),
        "LinComb" => LinearCombination::new(syntax.parameters[0],syntax.parameters[1],syntax.parameters[2],
            seq_from_syntax(&*syntax.sequences[0]),
            seq_from_syntax(&*syntax.sequences[1])),
        "Maximum" => Maximum::new(seq_from_syntax(&*syntax.sequences[0]),
            seq_from_syntax(&*syntax.sequences[1])),
        "PartialProduct" => PartialProduct::new(seq_from_syntax(&*syntax.sequences[0])),
        "Product" => Product::new(seq_from_syntax(&*syntax.sequences[0]),
            seq_from_syntax(&*syntax.sequences[1])),
        "Quadratic" => Quadratic::new(syntax.parameters[0],syntax.parameters[1],syntax.parameters[2],
            seq_from_syntax(&*syntax.sequences[0])),
        "Random" => Random::new(seq_from_syntax(&*syntax.sequences[0]),
            seq_from_syntax(&*syntax.sequences[1])),
        "Sum" => Sum::new(seq_from_syntax(&*syntax.sequences[0]),
            seq_from_syntax(&*syntax.sequences[1])),
        "Switch" => Switch::new(syntax.parameters[0],syntax.parameters[1],syntax.parameters[2],
            seq_from_syntax(&*syntax.sequences[0]),
            seq_from_syntax(&*syntax.sequences[1])),
        "WeightedAverage" => WeightedAverage::new(seq_from_syntax(&*syntax.sequences[0]),
            seq_from_syntax(&*syntax.sequences[1]),
            seq_from_syntax(&*syntax.sequences[2])),
        _ => panic!("Sequence does not exist: {}", syntax.name),
    }
}

pub async fn handle_request(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/ping") => Ok(Response::new(full(
            serde_json::to_string(&get_project()).unwrap(),
        ))),
        (&Method::GET, "/sequence") => {
            let sequences = sequences();
            Ok(Response::new(full(
                serde_json::to_string(&sequences).unwrap(),
            )))
        }
        (&Method::POST, r) => {
            let our_sequences = sequences();
            let sequence = our_sequences
                .iter()
                .find(|&x| ("/sequence/".to_string() + &x.name) == r);
            let body = collect_body(req).await?;
            let request: SequenceRequest = serde_json::from_str(&body).unwrap();
            let range = request.range;
            let wanted_sequences = request.sequences;
            let wanted_parameters = request.parameters;
            match sequence {
                Some(sequence_syntax) => {
                    let syntax = to_syntax(&sequence_syntax.name, wanted_parameters, wanted_sequences);
                    let seq = seq_from_syntax(&syntax);
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }
                None => create_404(),
            }
        }
        _ => create_404(),
    }
}
