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

use bytes::Bytes;
use hyper::{body::Incoming, Method, Request, Response};
use http_body_util::combinators::BoxBody;
use crate::info::sequences;
use crate::structs::sequence::SequenceRequest;
use crate::functions::project_handler::get_project;
use crate::functions::http_handler::{full,  collect_body, create_404, create_200};


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
            let seqs = sequences();
            let sequence = seqs
                .iter()
                .find(|&x| ("/sequence/".to_string() + &x.name) == r);
            match sequence {
                None => create_404(),
                Some(s) if *s.name == "Arithmetic".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Arithmetic::new(request.parameters[0], request.parameters[1]);
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }
                
                Some(s) if *s.name == "Chaos".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Chaos::new(request.parameters[0], request.parameters[1]);
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }

                Some(s) if *s.name == "Constant".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Constant::new(request.parameters[0]);
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }

                Some(s) if *s.name == "CrossProduct".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = CrossProduct::new(seq_from_syntax(&*request.sequences[0]), seq_from_syntax(&*request.sequences[1]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }

                Some(s) if *s.name == "Drop".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Drop::new(seq_from_syntax(&*request.sequences[0]), request.parameters[0]);
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }

                Some(s) if *s.name == "Fibonacci".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Fibonacci::new(seq_from_syntax(&*request.sequences[0]), seq_from_syntax(&*request.sequences[1]), seq_from_syntax(&*request.sequences[2]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }

                Some(s) if *s.name == "Geometric".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Geometric::new(request.parameters[0], request.parameters[1]);
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }

                Some(s) if *s.name == "LinComb".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = LinearCombination::new(request.parameters[0], request.parameters[1], request.parameters[2], seq_from_syntax(&*request.sequences[0]), seq_from_syntax(&*request.sequences[1]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }

                Some(s) if *s.name == "Maximum".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Maximum::new(seq_from_syntax(&*request.sequences[0]), seq_from_syntax(&*request.sequences[1]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }


                Some(s) if *s.name == "PartialProduct".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = PartialProduct::new(seq_from_syntax(&*request.sequences[0]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }


                Some(s) if *s.name == "Product".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Product::new(seq_from_syntax(&*request.sequences[0]), seq_from_syntax(&*request.sequences[1]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }


                Some(s) if *s.name == "Quadratic".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Quadratic::new(request.parameters[0], request.parameters[1], request.parameters[2], seq_from_syntax(&*request.sequences[0]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }

                Some(s) if *s.name == "Random".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Random::new(seq_from_syntax(&*request.sequences[0]), seq_from_syntax(&*request.sequences[1]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }


                Some(s) if *s.name == "Sum".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Sum::new(seq_from_syntax(&*request.sequences[0]), seq_from_syntax(&*request.sequences[1]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }


                Some(s) if *s.name == "Switch".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = Switch::new(request.parameters[0], request.parameters[1], request.parameters[2], seq_from_syntax(&*request.sequences[0]), seq_from_syntax(&*request.sequences[1]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }


                Some(s) if *s.name == "WeightedAverage".to_string() => {
                    let body = collect_body(req).await?;
                    let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                    let range = request.range;
                    let seq = WeightedAverage::new(seq_from_syntax(&*(request.sequences[0])), seq_from_syntax(&*request.sequences[1]), seq_from_syntax(&*request.sequences[2]));
                    Ok(Response::new(full(
                        serde_json::to_string(&seq.range(range)).unwrap(),
                    )))
                }

                _ => panic!("Not implemented"),
            }
        }
        _ => create_404(),
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
