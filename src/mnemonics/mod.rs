use anyhow::Result;


pub trait MnemonicFactory: Sized {
    fn generate12() -> Result<Self>;
    fn generate24() -> Result<Self>;
    fn from_phrase(phrase: &str) -> Result<Self>;

    /// Validate a mnemonic phrase
    ///
    /// The phrase supplied will be checked for word length and validated according to the checksum
    /// specified in BIP0039.
    fn validate(phrase: &str) -> Result<()>;
}

pub trait Mnemonic {
    fn phrase(&self) -> &str;
    fn into_phrase(self) -> String;
}

pub mod bip39;
