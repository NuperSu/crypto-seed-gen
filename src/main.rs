use anyhow::{Result};
use clap::{crate_version, App, Arg};
use thiserror::Error;
// use trompt::Trompt;

use crypto_seed_gen::{
    Bip39Mnemonic, Mnemonic, MnemonicFactory,
};
use std::sync::mpsc;
use std::thread;

// TODO This is only needed because trompt::Error doesn't implement std::error::TromptError. https://gitlab.com/runarberg/trompt/-/issues/4
#[derive(Debug, Error)]
pub enum TromptValidationError {
    #[error("absent")]
    Absent,
    #[error("too long")]
    TooLong,
    #[error("too short")]
    TooShort,
    #[error("unexpected input: {0}")]
    UnexpectedInput(String),
    #[error("other: {0}")]
    Other(String),
}
impl From<trompt::ValidationError> for TromptValidationError {
    fn from(err: trompt::ValidationError) -> TromptValidationError {
        match err {
            trompt::ValidationError::Absent => TromptValidationError::Absent,
            trompt::ValidationError::TooLong => TromptValidationError::TooLong,
            trompt::ValidationError::TooShort => TromptValidationError::TooShort,
            trompt::ValidationError::UnexpectedInput(input) => {
                TromptValidationError::UnexpectedInput(input)
            }
            trompt::ValidationError::Other(reason) => TromptValidationError::Other(reason),
        }
    }
}
#[derive(Debug, Error)]
pub enum TromptError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Validation(#[from] TromptValidationError),
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
}
impl From<trompt::Error> for TromptError {
    fn from(err: trompt::Error) -> TromptError {
        match err {
            trompt::Error::Io(err) => TromptError::Io(err),
            trompt::Error::Validation(err) => {
                TromptError::Validation(TromptValidationError::from(err))
            }
            trompt::Error::FromUtf8(err) => TromptError::FromUtf8(err),
        }
    }
}

fn fuck_you(mnemonic1: String) -> String {
    let mut mnemonic2 = Bip39Mnemonic::generate12().unwrap().phrase().to_string();
    let mut mnemonic3 = format!("{} {}", mnemonic1, mnemonic2);
    let mut mnemonic4 = format!("{} {}", mnemonic2, mnemonic1);
    while Bip39Mnemonic::from_phrase(mnemonic3.as_str()).is_err() || Bip39Mnemonic::from_phrase(mnemonic4.as_str()).is_err() {
        mnemonic2 = Bip39Mnemonic::generate12().unwrap().phrase().to_string();
        mnemonic3 = format!("{} {}", mnemonic1, mnemonic2);
        mnemonic4 = format!("{} {}", mnemonic2, mnemonic1);
    }
    mnemonic2
}

fn main() -> Result<()> {
    let args = App::new("Crypto Seed Generator")
        .version(crate_version!())
        .author("Anton Kosovskii <antonkosovsk@gmail.com>")
        .about("Generates 4 cool mnemonic seeds")
        .arg(
            Arg::with_name("from-mnemonic")
                .short("m")
                .long("from-mnemonic")
                .value_name("MNEMONIC SEED PHRASE 12 WORD LONG")
                .case_insensitive(true)
                .help("The mnemonic seed phrase to use to generate cool phrases"),
        )
        .get_matches();
    let mnemonic1 = args.value_of("from-mnemonic");
    let mnemonic1: String = match mnemonic1 {
        Some(mnemonic1) => (Bip39Mnemonic::from_phrase(mnemonic1)?).phrase().to_string(),
        None => Bip39Mnemonic::generate12()?.phrase().to_string(),
    };
    if mnemonic1.split(" ").count() != 12 {
        println!("User entered mnemonic seed phrase must be 12 words long");
        std::process::exit(1);
    }
    let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();
        let mnemonic1_clone = mnemonic1.clone();
        thread::spawn(move || {
            let mnemonic2 = fuck_you(mnemonic1_clone);
            tx_clone.send(mnemonic2).unwrap();
        });
        let tx_clone = tx.clone();
        let mnemonic1_clone = mnemonic1.clone();
        thread::spawn(move || {
            let mnemonic2 = fuck_you(mnemonic1_clone);
            tx_clone.send(mnemonic2).unwrap();
        });
        let tx_clone = tx.clone();
        let mnemonic1_clone = mnemonic1.clone();
        thread::spawn(move || {
            let mnemonic2 = fuck_you(mnemonic1_clone);
            tx_clone.send(mnemonic2).unwrap();
        });
        let tx_clone = tx.clone();
        let mnemonic1_clone = mnemonic1.clone();
        thread::spawn(move || {
            let mnemonic2 = fuck_you(mnemonic1_clone);
            tx_clone.send(mnemonic2).unwrap();
        });
        let tx_clone = tx.clone();
        let mnemonic1_clone = mnemonic1.clone();
        thread::spawn(move || {
            let mnemonic2 = fuck_you(mnemonic1_clone);
            tx_clone.send(mnemonic2).unwrap();
        });
        let tx_clone = tx.clone();
        let mnemonic1_clone = mnemonic1.clone();
        thread::spawn(move || {
            let mnemonic2 = fuck_you(mnemonic1_clone);
            tx_clone.send(mnemonic2).unwrap();
        });
        let tx_clone = tx.clone();
        let mnemonic1_clone = mnemonic1.clone();
        thread::spawn(move || {
            let mnemonic2 = fuck_you(mnemonic1_clone);
            tx_clone.send(mnemonic2).unwrap();
        });
        let tx_clone = tx.clone();
        let mnemonic1_clone = mnemonic1.clone();
        thread::spawn(move || {
            let mnemonic2 = fuck_you(mnemonic1_clone);
            tx_clone.send(mnemonic2).unwrap();
        });
    let mnemonic2 = rx.recv().unwrap();
    let mnemonic3 = format!("{} {}", mnemonic1, mnemonic2);
    let mnemonic4 = format!("{} {}", mnemonic2, mnemonic1);

    println!(
        "Phrase1: {}\nPhrase2: {}\nPhrase1+Phrase2: {}\nPhrase2+Phrase1: {}",
        mnemonic1,
        mnemonic2,
        mnemonic3,
        mnemonic4
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electrum_derivation_matches_bip44() {
        // Test that when importing a derived key into electrum, electrum generates the correct BIP44 keys.
        // To test this, we generated a mnemonic at https://iancoleman.io/bip39/
        let mnemonic = "giggle load civil velvet legend drink letter symbol vivid tube parent plug accuse fault choose ahead bomb make novel potato enrich honey cable exchange";
        // We then use our tool to generate the private key
        let master_seed = Bip39Mnemonic::from_phrase(mnemonic)
            .unwrap()
            .to_private_key("")
            .unwrap();
        assert_eq!(
            "xprv9zEiTz4LvP1k9brLSck5yX41EzVi3xbC2ZkPhWdyTqvJu3ovQCD6R8Z8RUoTwKkwpdqMne95zSrk9duV2SYhmmRkxvZAMsdqNHThKP8STbi",
            derive_key(master_seed, Bip44DerivationPath {
                coin_type: CoinType::BTC, account: 0, change: None, address_index: None}).unwrap().to_base58(),
        );
        // and loaded that key into electrum, checking that electrum generates the BIP44 addresses
        // listed on https://iancoleman.io/bip39/
        // So this test case is basically a test ensuring that we keep generating the same private key for which we already checked
        // what electrum generates from it and don't start differring from it.
    }
}
