use crate::structs::sequence::SequenceInfo;

pub fn sequences() -> Vec<SequenceInfo> {
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Arithmetic".to_string(),
        description: "Arithmetic sequence".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Constant".to_string(),
        description: "".to_string(),
        parameters: 1,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Cross product".to_string(),
        description: "".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Drop".to_string(),
        description: "".to_string(),
        parameters: 1,
        sequences: 1,
    });
    sequences.push(SequenceInfo {
        name: "Eval".to_string(),
        description: "".to_string(),
        parameters: 0,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Geometris".to_string(),
        description: "".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Partial product".to_string(),
        description: "".to_string(),
        parameters: 0,
        sequences: 1,
    });
    sequences.push(SequenceInfo {
        name: "Product".to_string(),
        description: "".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Random".to_string(),
        description: "".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Sum".to_string(),
        description: "".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Lin Comb".to_string(),
        description: "".to_string(),
        parameters: 3,
        sequences: 2,
    });

    sequences
}