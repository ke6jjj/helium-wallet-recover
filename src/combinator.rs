
use super::spelling::AlternativeTable;
use anyhow::{Result, anyhow};
use helium_mnemonic::Language;


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
    fn new(radix: &Vec<u64>, combinator: &'a Combinator) -> Self {
        let mut max: u64 = 1;
        for v in radix.iter() {
            max *= v
        }
        Self {
            combinator,
            i: 0,
            max: max, 
            radix: radix.clone(),
        }
    }

    fn sequence_from_index(&self, index: u64) -> Vec<u64> {
        let mut i = index;
        let mut r: Vec<u64> = Vec::with_capacity(self.radix.len());
        for depth in self.radix.iter() {
            let this = i % depth;
            i = i / depth;
            r.push(this);
        }
        r
    }

    pub fn size(&self) -> u64 {
        self.max
    }
}

impl<'a> Iterator for CombinatorIterator<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.max {
            return None
        }
        let r = self.sequence_from_index(self.i);
        self.i += 1;
        Some(self.combinator.words(&r))
    }
}

impl Combinator {
    pub fn new_from_list(list: &Vec<&str>, distance: usize) -> Result<Combinator> {
        let english = Language::English;
        let ok = list
            .iter()
            .map(|word| english.find_word(*word))
            .all(|r| r.is_some());
        if !ok {
            return Err(anyhow!("invalid word in list"))
        }
        let x = AlternativeTable::new(distance);
        let mut word_plan: Vec<Vec<String>> = Vec::with_capacity(list.len());
        for word in list {
            let alternatives: Vec<String> = x.alternatives(*word)
                .unwrap()
                .iter()
                .map(|x| x.to_owned())
                .collect();
            word_plan.push(alternatives);
        }
        
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
        CombinatorIterator::new(&substitions_by_word, self)
    }

    /// Construct a phrase choice from an iteration vector.
    pub fn words(&self, the_indexes: &Vec<u64>) -> Vec<&str> {
        let res: Vec<&str> = the_indexes
            .iter()
            .enumerate()
            .map(|(word_idx, choice_idx)| self.plan[word_idx][*choice_idx as usize].as_str())
            .collect();
        res
    }
}

#[cfg(test)]
mod test {
    use super::Combinator;

    #[test]
    fn test_one() {
        let combo = Combinator::new_from_list(&vec![ "derp" ], 1);
        assert!(combo.is_err())
    }
}