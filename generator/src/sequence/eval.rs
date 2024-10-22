use crate::sequence::Sequence;

pub struct EvalSequence<S, F> {
    sequence: S,
    func: F,
}

impl<S: Sequence<i64>, F: Fn(i64) -> i64> EvalSequence<S, F> {
    pub fn new(sequence: S, func: F) -> EvalSequence<S, F> {
        EvalSequence { sequence, func }
    }
}

impl<S: Sequence<i64>, F: Fn(i64) -> i64> Sequence<i64> for EvalSequence<S, F> {
    fn name(&self) -> String {
        format!("Evaluated sequence of {}", self.sequence.name())
    }

    fn contains(&self, item: i64) -> bool {
        let mut i = 0;
        while let Some(term) = self.sequence.k_th(i) {
            if (self.func)(term) == item {
                return true;
            }
            i += 1;
        }
        false
    }

    fn k_th(&self, k: i64) -> Option<i64> {
        self.sequence.k_th(k).map(|x| (self.func)(x))
    }

    fn start(&self) -> Option<i64> {
        self.k_th(0)
    }
}