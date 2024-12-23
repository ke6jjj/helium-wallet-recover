mod spelling;
mod combinator;
mod reading;

use combinator::Combinator;
use helium_lib::keypair::Signer;
use helium_lib::error::Error;
use helium_mnemonic::MnmemonicError;
use reading::generate_readings;
use std::env::args;
use anyhow::{Result, Context};


fn main() -> Result<()> {
    let crap: Vec<String> = 
        args()
        .into_iter()
        .skip(1)
        .collect();
    let seed_words: Vec<&str> = crap.iter().map(|x| x.as_str()).collect();

    let spelling_alternatives = Combinator::new_from_list(&seed_words, 1)
        .context("error with word list")?;

    let alternatives = spelling_alternatives.choice_iter();
    let mistaken_readings = generate_readings(12)?;
    let total_combinations = alternatives.size() * (mistaken_readings.len() as u64);
    println!("Trying {} combinations.", total_combinations);

    let mut i = 1;

    for spelling_vec in alternatives {
        let words = spelling_alternatives.words(&spelling_vec);
        for reading_vec in mistaken_readings.iter() {
            let reading: Vec<String> = 
                reading_vec
                .iter()
                .map(|position| words[*position as usize].to_owned())
                .collect();
            let keypair_res = helium_lib::keypair::Keypair::from_words(reading.clone());
            let phrase = reading.join(" ");
            match keypair_res {
                Ok(keypair) => println!("{i}. {}: OK {}", phrase, keypair.pubkey().to_string()),
                Err(Error::Mnemonic(MnmemonicError::InvalidChecksum)) => println!("{i}. {}: XX Invalid", phrase),
                other => { other.context("decoding key")?; () }
            }
            i += 1
        }
    }
    Ok(())
}
