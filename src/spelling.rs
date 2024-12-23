use std::collections::HashMap;
use helium_mnemonic::Language;
use fuzzt::algorithms::damerau_levenshtein;

pub struct AlternativeTable {
    map: HashMap<String, Vec<String>>
}

impl AlternativeTable {
    pub fn new(max_distance: usize) -> Self {
        let mut the_map = HashMap::new();
        for i in 0..2048 {
            let a = &Language::English[i as usize];
            let mut words: Vec<String> = Vec::with_capacity(1);
            words.push(a.to_owned());
            for j in 0..2048 {
                if j == i {
                    continue
                }
                let b = &Language::English[j as usize];
                let distance = damerau_levenshtein(a, b);
                if distance <= max_distance {
                    words.push(b.to_owned())
                }
            }
            the_map.insert(a.to_owned(), words);
        }
        AlternativeTable { map: the_map }
    }

    pub fn alternatives(&self, s: &str) -> Option<&Vec<String>> {
        self.map.get(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use helium_mnemonic::Language;

    #[test]
    fn it_works() {
        let gen = AlternativeTable::new(1);
        let x = gen.alternatives("able").unwrap();
        for word in x {
            println!("{}", word);
        }
    }

    #[test]
    fn generate_md_table() {
        let gen = AlternativeTable::new(1);
        println!("|Word|Alternatives|");
        println!("|---|---|");
        for j in 0..2048 {
            let word = &Language::English[j as usize];
            let with_alternates = gen.alternatives(word).expect("huh?");
            if with_alternates.len() < 2 {
                continue
            }
            let alternates: Vec<String> = with_alternates
                .iter().skip(1).map(|x| x.to_owned()).collect();
            println!("|{}|{}|", word, alternates.join(" "));
        }
    }
}
