mod spelling;
mod combinator;

use combinator::Combinator;
use helium_lib::keypair::Signer;
use std::env::args;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let crap: Vec<String> = args().into_iter().collect();
    let crap_ref: Vec<&str> = crap[1..].iter().map(|x| x.as_str()).collect();
    let x = crap_ref.join(" ");
    println!("Got {}", x);
    let comber = Combinator::new_from_list(&crap_ref, 1)
        .context("error with word list")?;
    let combos = comber.iter();
    let remaps: Vec<Vec<usize>> = vec![
        vec![ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ],
        vec![ 6, 7, 8, 9, 10, 11, 0, 1, 2, 3, 4, 5 ],
        vec![ 0, 6, 1, 7, 2, 8, 3, 9, 4, 10, 5, 11 ],
        vec![ 6, 0, 7, 1, 8, 2, 9, 3, 10, 4, 11, 5 ],
        vec![ 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 ],
        vec![ 5, 4, 3, 2, 1, 0, 11, 10, 9, 8, 7, 6 ],
        vec![ 11, 5, 10, 4, 9, 3, 8, 2, 7, 1, 6, 0 ],
        vec![ 5, 11, 4, 10, 3, 9, 2, 8, 1, 7, 0, 6 ],
    ];
    let combo_iter = combos.into_iter();
    println!("Trying {} combinations.", combo_iter.size() * (remaps.len() as u64));
    let mut i = 1;
    for combo in combo_iter {
        let first_level = comber.words(&combo);
        for remap in remaps.iter() {
            let the_words: Vec<&str> = remap.iter().map(|x| first_level[*x]).collect();
            let goddammit: Vec<String> = the_words.iter().map(|s| (*s).to_owned()).collect();
            let keypair_res = helium_lib::keypair::Keypair::from_words(goddammit);
            let x = the_words.join(" ");
            if keypair_res.is_ok() {
                let keypair = keypair_res.unwrap();
                println!("{i}. {}: OK {}", x, keypair.pubkey().to_string());
            } else {
                println!("{i}. {}: XX Invalid", x);
            }
            i += 1
        }
    }
    Ok(())
}
