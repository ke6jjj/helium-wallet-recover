mod combinator;
mod reading;
mod spelling;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use combinator::Combinator;
use helium_lib::error::Error;
use helium_lib::keypair::{Pubkey, Signer};
use helium_mnemonic::MnmemonicError;
use reading::generate_readings;

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
    words: Vec<String>,
}

struct Verbosity {
    verbose: bool,
}

impl Verbosity {
    pub fn verbose(v: bool) -> Verbosity {
        Verbosity { verbose: v }
    }

    pub fn show<F: FnOnce()>(&self, f: F) {
        if self.verbose {
            f()
        }
    }
}

/// Given a nominal reading of a seed phrase, return an alternate reading
/// of the phrase, with the words swapped according to the provided reading
/// order schedule.
fn phrase_from_misreading<'a>(phrase: &[ &'a str], reading_order: &[u8]) -> Vec<&'a str> {
    let reassembled: Vec<&str> = reading_order
        .iter()
        .map(|&position| phrase[position as usize])
        .collect();
    reassembled
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let seed_words: Vec<&str> = args.words.iter().map(|x| x.as_str()).collect();
    let v = Verbosity::verbose(args.verbose || args.dry_run);
    let t = Verbosity::verbose(args.target.is_none() || args.verbose);

    if seed_words.len() != 12 {
        return Err(anyhow!("Incorrect number of seed words. Need 12."));
    }

    v.show(|| {
        eprint!(
            "Building word distance dictionary with limit {}...",
            args.distance
        )
    });
    let spelling_alternatives =
        Combinator::new_from_list(&seed_words, 1).context("Seed phrase unusable")?;
    v.show(|| eprintln!("done."));

    let alternatives = spelling_alternatives.iter();
    let mistaken_readings = generate_readings(12)?;
    let total_combinations = alternatives.size() * (mistaken_readings.len() as u64);
    v.show(|| eprintln!("Trying {} combinations.", total_combinations));

    if args.dry_run {
        return Ok(());
    }

    let mut i = 1;

    for alternative_phrase in alternatives {
        for reading in mistaken_readings.iter() {
            let candidate_phrase = phrase_from_misreading(&alternative_phrase, reading);
            let keypair_res = helium_lib::keypair::Keypair::from_words(&candidate_phrase);
            let as_single_string = candidate_phrase.join(" ");
            match keypair_res {
                Ok(keypair) => {
                    let pubkey = keypair.pubkey();
                    t.show(|| eprintln!("{i}. {}: OK {}", as_single_string, pubkey));
                    if args.target.is_some_and(|t| t == pubkey) {
                        println!("{i}. {}: FOUND {}", as_single_string, pubkey);
                        return Ok(());
                    }
                }
                Err(Error::Mnemonic(MnmemonicError::InvalidChecksum)) => {
                    v.show(|| eprintln!("{i}. {}: XX Invalid", as_single_string))
                }
                other => {
                    other.context("decoding key")?;
                    
                }
            }
            i += 1
        }
    }
    Ok(())
}
