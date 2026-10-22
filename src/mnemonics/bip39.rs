use anyhow::Result;
use bip39::{Language, Mnemonic as _Mnemonic};
use rand::RngCore;

use super::{Mnemonic, MnemonicFactory};
use crate::random::secure_rng;

const LANG: Language = Language::English;

#[derive(Debug)]
pub struct Bip39Mnemonic {
    // wagyu_bitcoin::mnemonic::BitcoinMnemonic::to_seed() is private, so we need to use the bip39 crate instead.
    mnemonic: _Mnemonic,
}

impl MnemonicFactory for Bip39Mnemonic {
    fn generate12() -> Result<Self> {
        const ENTROPY_LENGTH: usize = 16;
        // XOR an OS rng and a pseudo rng to get our entropy. Probably not necessary but doesn't hurt either.
        let mut rng = secure_rng()?;
        let mut entropy: [u8; ENTROPY_LENGTH] = [0; ENTROPY_LENGTH];
        rng.fill_bytes(&mut entropy);
        let mnemonic = _Mnemonic::from_entropy(&entropy, LANG).expect("Invalid key length");
        Ok(Self { mnemonic })
    }
    fn generate24() -> Result<Self> {
        const ENTROPY_LENGTH: usize = 32;
        // XOR an OS rng and a pseudo rng to get our entropy. Probably not necessary but doesn't hurt either.
        let mut rng = secure_rng()?;
        let mut entropy: [u8; ENTROPY_LENGTH] = [0; ENTROPY_LENGTH];
        rng.fill_bytes(&mut entropy);
        let mnemonic = _Mnemonic::from_entropy(&entropy, LANG).expect("Invalid key length");
        Ok(Self { mnemonic })
    }

    fn from_phrase(phrase: &str) -> Result<Self> {
        let mnemonic = _Mnemonic::from_phrase(phrase, LANG)?;
        Ok(Self { mnemonic })
    }

    fn validate(phrase: &str) -> Result<()> {
        _Mnemonic::validate(phrase, LANG)
    }
}

impl Mnemonic for Bip39Mnemonic {
    fn phrase(&self) -> &str {
        self.mnemonic.phrase()
    }

    fn into_phrase(self) -> String {
        self.mnemonic.into_phrase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_phrase_is_24_words() {
        let phrase = Bip39Mnemonic::generate24().unwrap().into_phrase();
        assert_eq!(23, phrase.chars().filter(|a| *a == ' ').count());
    }

    #[test]
    fn generated_phrase_is_valid() {
        Bip39Mnemonic::validate(Bip39Mnemonic::generate24().unwrap().phrase()).unwrap();
    }

    #[test]
    fn validate_valid_24word_phrase() {
        Bip39Mnemonic::validate("desert armed renew matrix congress order remove lab travel shallow there tool symbol three radio exhibit pledge alcohol quit host rare noble dose eager").unwrap();
    }

    #[test]
    fn validate_valid_21word_phrase() {
        Bip39Mnemonic::validate("morning mind present cloud boat phrase task uniform effort couple carpet wise steak eyebrow friend birth million photo tobacco firm hobby").unwrap();
    }

    #[test]
    fn validate_valid_18word_phrase() {
        Bip39Mnemonic::validate("slice lift violin movie shield copy tail arrow idle lift knock fossil leave lawsuit tennis sight travel vivid").unwrap();
    }

    #[test]
    fn validate_valid_15word_phrase() {
        Bip39Mnemonic::validate("call oval opinion exhibit limit write fine prepare sleep possible extend language split kidney desert").unwrap();
    }

    #[test]
    fn validate_valid_12word_phrase() {
        Bip39Mnemonic::validate(
            "tornado ginger error because arrange lake scale unfold palm theme frozen sick",
        )
        .unwrap();
    }

    #[test]
    fn validate_invalid_20word_phrase() {
        let err = Bip39Mnemonic::validate(
            "morning mind present cloud boat phrase task uniform effort couple carpet wise steak eyebrow friend birth million photo tobacco firm",
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("invalid number of words in phrase"))
    }

    #[test]
    fn validate_invalid_21word_phrase() {
        let err = Bip39Mnemonic::validate(
            "morning mind present cloud boat phrase task uniform effort couple carpet wise steak eyebrow friend birth million photo tobacco firm prepare",
        )
        .unwrap_err();
        assert!(err.to_string().contains("invalid checksum"))
    }

    #[test]
    fn from_invalid_20word_phrase() {
        let err = Bip39Mnemonic::from_phrase(
            "morning mind present cloud boat phrase task uniform effort couple carpet wise steak eyebrow friend birth million photo tobacco firm",
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("invalid number of words in phrase"))
    }

    #[test]
    fn from_invalid_21word_phrase() {
        let err = Bip39Mnemonic::from_phrase(
            "morning mind present cloud boat phrase task uniform effort couple carpet wise steak eyebrow friend birth million photo tobacco firm prepare",
        )
        .unwrap_err();
        assert!(err.to_string().contains("invalid checksum"))
    }
}
