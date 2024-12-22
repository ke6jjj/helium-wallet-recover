use super::spelling::AlternativeTable;
use anyhow::{Result, Context, anyhow};
use helium_mnemonic::Language;


pub struct Combinator {
    plan: Vec<Vec<String>>,
}

pub struct RadixIterator {
    i: u64,
    max: u64,
    radix: Vec<u64>,
}

impl RadixIterator {
    pub fn new(radix: &Vec<u64>) -> Self {
        let mut max: u64 = 1;
        for v in radix.iter() {
            max *= v
        }
        Self {
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
}

impl Iterator for RadixIterator {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.max {
            return None
        }
        let r = self.sequence_from_index(self.i);
        self.i += 1;
        Some(r)
    }
}

impl Combinator {
    pub fn new_from_list<'a>(list: &Vec<&'a str>) -> Result<Combinator> {
        let english = Language::English;
        let ok = list
            .iter()
            .map(|word| english.find_word(*word))
            .all(|r| r.is_some());
        if !ok {
            return Err(anyhow!("invalid word in list"))
        }
        let x = AlternativeTable::new();
        let mut word_plan: Vec<Vec<String>> = Vec::with_capacity(list.len());
        for word in list {
            let alternatives = x.alternatives(*word);
            let mut plan = vec![ (*word).to_owned() ];
            match alternatives {
                None => {},
                Some(alternates) => {
                    for alternate in alternates {
                        plan.push((*alternate).to_owned());
                    }
                }
            }
            word_plan.push(plan);
        }
        
        Ok(Combinator {
            plan: word_plan,
        })
    }

    pub fn iter(&self) -> RadixIterator {
        let substitions_by_word: Vec<u64> = self
            .plan
            .iter()
            .map(|plan| plan.len() as u64)
            .collect();
        RadixIterator::new(&substitions_by_word)
    }

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
        let combo = Combinator::new_from_list(&vec![ "derp" ]);
        assert!(combo.is_err())
    }
}