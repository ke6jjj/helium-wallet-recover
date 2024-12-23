mod spelling;
mod combinator;
mod reading;

use combinator::Combinator;
use helium_lib::keypair::{Pubkey, Signer};
use helium_lib::error::Error;
use helium_mnemonic::MnmemonicError;
use reading::generate_readings;
use anyhow::{anyhow, Context, Result};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(name = env!("CARGO_BIN_NAME"))]
/// Exhaustive search to recover seed phrase mispellings for Helium L1 wallets.
/// 
/// This program allows one to attempt to recover a Helium wallet from a
/// mnemonic seed phrase that has misspellings and/or was written down
/// in the wrong order.
pub struct Cli {
    /// Spelling distance metric
    #[arg(short = 'd', long, default_value_t = 1)]
    distance: u8,

    /// Target public key (if known)
    #[arg(short = 't', long)]
    target: Option<Pubkey>,

    /// Show all combinations tried, even ones with invalid checksums.
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Show what would be done then exit quickly.
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Passphrase seed words
    words: Vec<String>
}

struct Verbosity {
    verbose: bool
}

impl Verbosity {
    pub fn verbose(v: bool) -> Verbosity {
        Verbosity { verbose: v }
    }

    pub fn show<F: FnOnce() -> ()>(&self, f: F) {
        if self.verbose {
            f()
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let seed_words: Vec<&str> = args.words.iter().map(|x| x.as_str()).collect();
    let v = Verbosity::verbose(args.verbose || args.dry_run);
    let t = Verbosity::verbose(args.target.is_none() || args.verbose);

    if seed_words.len() != 12 {
        return Err(anyhow!("Incorrect number of seed words. Need 12."));
    }

    v.show(|| print!("Building word distance dictionary with limit {}...", args.distance));
    let spelling_alternatives = Combinator::new_from_list(&seed_words, 1)
        .context("error with word list")?;
    v.show(|| println!("done."));

    let alternatives = spelling_alternatives.choice_iter();
    let mistaken_readings = generate_readings(12)?;
    let total_combinations = alternatives.size() * (mistaken_readings.len() as u64);
    v.show(|| println!("Trying {} combinations.", total_combinations));

    if args.dry_run {
        return Ok(())
    }

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
                Ok(keypair) => {
                    let pubkey = keypair.pubkey();
                    t.show(|| println!("{i}. {}: OK {}", phrase, pubkey.to_string()));
                    if args.target.is_some_and(|t| t == pubkey) {
                        println!("{i}. {}: FOUND {}", phrase, pubkey.to_string());
                        return Ok(())
                    }
                },
                Err(Error::Mnemonic(MnmemonicError::InvalidChecksum)) => v.show(|| println!("{i}. {}: XX Invalid", phrase)),
                other => { other.context("decoding key")?; () }
            }
            i += 1
        }
    }
    Ok(())
}
