use crate::sequence::Sequence;

pub struct CrossProduct<S1, S2> {
    sequence1: S1,
    sequence2: S2,
}

impl<S1: Sequence<i64>, S2: Sequence<i64>> CrossProduct<S1, S2> {
    pub fn new(sequence1: S1, sequence2: S2) -> CrossProduct<S1, S2> {
        CrossProduct { sequence1, sequence2 }
    }
}

impl<S1: Sequence<i64>, S2: Sequence<i64>> Sequence<i64> for CrossProduct<S1, S2> {
    fn k_th(&self, k: usize) -> Option<i64> {
        // Pridobimo a_n, a_(n+1), b_n, b_(n+1)
        let a_n = self.sequence1.k_th(k as i64)?;
        let a_n1 = self.sequence1.k_th((k + 1) as i64)?;
        let b_n = self.sequence2.k_th(k as i64)?;
        let b_n1 = self.sequence2.k_th((k + 1) as i64)?;

        //"križni produkt" dobimo po rekurziji c_n = a_n * b_n1 + a_n1 * b_n
        Some(a_n * b_n1 + a_n1 * b_n)
    }

    fn name(&self) -> String {
        format!(
            "Cross product of {} and {}",
            self.sequence1.name(),
            self.sequence2.name()
        )
    }

    fn contains(&self, item: i64) -> bool {
        let mut k = 0;
        while let Some(c_n) = self.k_th(k) {
            if c_n == item {
                return true;
            }
            k += 1;
        }
        false
    }

    fn start(&self) -> Option<i64> {
        self.k_th(0)
    }
}