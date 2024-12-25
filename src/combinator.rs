
use super::spelling::AlternativeTable;
use helium_mnemonic::Language;
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CombinatorError {
    #[error("invalid seed word \"{1}\" at index {0}")]
    InvalidSeedWord(usize, String),
}

pub struct Combinator {
    plan: Vec<Vec<String>>,
}

pub struct CombinatorIterator<'a> {
    combinator: &'a Combinator,
    i: u64,
    max: u64,
    radix: Vec<u64>,
}

impl<'a> CombinatorIterator<'a> {
    fn new(combinator: &'a Combinator, radix: Vec<u64>) -> Self {
        // Compute the product of all the choices that could be made.
        let max = radix.iter().fold(1, |res, a| res * a);
        Self {
            combinator,
            i: 0,
            max, 
            radix,
        }
    }

    pub fn size(&self) -> u64 {
        self.max
    }
}

impl<'a> Iterator for CombinatorIterator<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.i;
        if i == self.max {
            return None
        }
        let mut r: Vec<&str> = Vec::with_capacity(self.radix.len());
        r.extend(
            self.radix
            .iter()
            .zip_eq(&self.combinator.plan)
            .map(|(depth, alternatives)| {
                let word_choice = i % *depth;
                i = i / *depth;
                alternatives[word_choice as usize].as_str()
            })
        );
        self.i += 1;
        Some(r)
    }
}

impl Combinator {
    pub fn new_from_list(list: &Vec<&str>, distance: usize) -> Result<Combinator, CombinatorError> {
        let english = Language::English;
        list
            .iter()
            .enumerate()
            .try_for_each(|(idx, &word)| {
                english
                    .find_word(word)
                    .map(|_| ())
                    .ok_or_else(|| CombinatorError::InvalidSeedWord(idx, word.to_string()))
            })?;
        let x = AlternativeTable::new(distance);
        let mut word_plan: Vec<Vec<String>> = Vec::with_capacity(list.len());
        word_plan.extend(
            list
            .iter()
            .map(|&word| {
                x.alternatives(word)
                    .unwrap()
                    .iter()
                    .map(|x| x.to_owned())
                    .collect()
            })
        );
        
        Ok(Combinator {
            plan: word_plan,
        })
    }

    /// Yield an iterator which iterates over all the different combinations
    /// of word choices for the full phrase given, allowing for the number
    /// plausible mistakes per-word given.
    pub fn iter(&self) -> CombinatorIterator {
        let substitions_by_word: Vec<u64> = self
            .plan
            .iter()
            .map(|plan| plan.len() as u64)
            .collect();
        CombinatorIterator::new(self, substitions_by_word)
    }
}

#[cfg(test)]
mod test {
    use crate::combinator::CombinatorError;

    use super::Combinator;

    #[test]
    fn test_one() {
        let combo = Combinator::new_from_list(&vec![ "derp" ], 1);
        assert!(combo.is_err())
    }

    #[test]
    fn test_error_locator() {
        let combo_res = Combinator::new_from_list(&vec![ "tiny", "derp" ], 1);
        let ok = match combo_res {
            Err(CombinatorError::InvalidSeedWord(idx, word)) => {
                assert_eq!(idx, 1);
                assert_eq!(word, "derp");
                true
            },
            Ok(_) => false
        };
        assert!(ok)
    }

}