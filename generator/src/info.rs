use crate::structs::sequence::SequenceInfo;

pub fn sequences() -> Vec<SequenceInfo> {
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Arithmetic".to_string(),
        description: "Arithmetic sequence with two parameters: the starting element and the step.".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Chaos".to_string(),
        description: "Sequence of chaos takes two parameters, the starting element and the parameter of chaos, 
        and returns the next element according to the formula: ck = r * ck-1 * (1 - ck-1).".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Constant".to_string(),
        description: "Constant sequence of a given parameter: constant.".to_string(),
        parameters: 1,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "CrossProduct".to_string(),
        description: "Cross product sequence takes two sequences ak and bk 
        and returns the next element as the cross product of ak * bk+1 + ak+1 * bk.".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Drop".to_string(),
        description: "Drop sequences is the given sequence, shifted by the parameter delay.".to_string(),
        parameters: 1,
        sequences: 1,
    });
    sequences.push(SequenceInfo {
        name: "Fibonacci".to_string(),
        description: "Fibonacci sequence takes two sequences and as the next element returns the sum of previous two elements,
        each multiplied by one of the sequences.".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Geometric".to_string(),
        description: "Geometric sequence with two parameters: the starting element and the ratio.".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "LinComb".to_string(),
        description: "Linear combination calculated by the formula: a * an + b * bn + c, 
        where a, b, c are parameters and an, bn are sequences".to_string(),
        parameters: 3,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Maximum".to_string(),
        description: "Maximum sequence takes two sequences and returns their maximum.".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "PartialProduct".to_string(),
        description: "Partial product sequence returns products of all previous element of the given sequence.".to_string(),
        parameters: 0,
        sequences: 1,
    });
    sequences.push(SequenceInfo {
        name: "Product".to_string(),
        description: "Product sequence takes two sequences and returns their product.".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Quadratic".to_string(),
        description: "Quadratic sequence is calculated according to the quadratic function: ax^2 + bx + c, 
        where a, b, c are parameters and x is the k-th element of the given sequence.".to_string(),
        parameters: 3,
        sequences: 1,
    });
    sequences.push(SequenceInfo {
        name: "Random".to_string(),
        description: "Random sequence returns a random number between k-th elements of both sequences.".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Sum".to_string(),
        description: "Sum sequence takes two sequences and returns their sum.".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Switch".to_string(),
        description: "Switch sequences takes three parameters, the lower and upper limit and the switch. 
        If the random number between limits is lower than the switch, it returns the k-th element of the first sequence
        and if it is higher the k-th element of the second sequence.".to_string(),
        parameters: 3,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "WeightedAverage".to_string(),
        description: "Weighted avarage is calculated by formula: wk * ak + (1-wk) * bk, where wk is the weighted sequence
        and ak, bk are two other sequences.".to_string(),
        parameters: 0,
        sequences: 3,
    });
    sequences
}