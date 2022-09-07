use anyhow::{Result};
use clap::{crate_version, App, Arg};
use thiserror::Error;
// use trompt::Trompt;

use crypto_wallet_gen::{
    Bip39Mnemonic, Mnemonic, MnemonicFactory,
};

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

fn main() -> Result<()> {
    let args = App::new("Crypto Wallet Generator")
        .version(crate_version!())
        .author("Sebastian Messmer <mail@smessmer.de>")
        .about("Generates crypto currency wallets from mnemonic seeds")
        .arg(
            Arg::with_name("from-mnemonic")
                .short("m")
                .long("from-mnemonic")
                .value_name("MNEMONIC SEED PHRASE")
                .case_insensitive(true)
                .help("The mnemonic seed phrase to use to generate the wallet"),
        )
        .get_matches();
    let mnemonic = args.value_of("from-mnemonic");
    let mnemonic: Box<dyn Mnemonic> =
        Box::new(
            mnemonic
                .map(|m| Bip39Mnemonic::from_phrase(m))
                .unwrap_or_else(Bip39Mnemonic::generate12)?,
        );

    println!(
        "Mnemonic: {}\nPassword: [omitted from output]",
        mnemonic.phrase()
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
