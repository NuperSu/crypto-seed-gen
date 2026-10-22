use anyhow::{anyhow, ensure, Context, Result};
use clap::{crate_version, App, Arg};
use thiserror::Error;
// use trompt::Trompt;

use crypto_seed_gen::{Bip39Mnemonic, Mnemonic, MnemonicFactory};
use std::sync::atomic::{AtomicBool, Ordering};
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

/// Search until a compatible phrase is found or another worker finishes.
fn find_compatible_mnemonic(mnemonic1: &str, cancelled: &AtomicBool) -> Result<Option<String>> {
    while !cancelled.load(Ordering::Relaxed) {
        let mnemonic2 = Bip39Mnemonic::generate12()?.into_phrase();
        if Bip39Mnemonic::validate(&format!("{} {}", mnemonic1, mnemonic2)).is_ok()
            && Bip39Mnemonic::validate(&format!("{} {}", mnemonic2, mnemonic1)).is_ok()
        {
            return Ok(Some(mnemonic2));
        }
    }
    Ok(None)
}

fn generate_compatible_mnemonic(mnemonic1: &str, workers: usize) -> Result<String> {
    run_search_workers(mnemonic1, workers, find_compatible_mnemonic)
}

fn run_search_workers<F>(mnemonic1: &str, workers: usize, search: F) -> Result<String>
where
    F: Fn(&str, &AtomicBool) -> Result<Option<String>> + Sync + std::panic::RefUnwindSafe,
{
    ensure!(workers > 0, "Number of threads must be greater than 0");
    let cancelled = AtomicBool::new(false);
    thread::scope(|scope| {
        // At most one message per worker; sending never waits for the receiver.
        let (tx, rx) = mpsc::sync_channel(workers);
        let mut handles = Vec::with_capacity(workers);
        let mut spawn_error = None;
        for index in 0..workers {
            let tx = tx.clone();
            let cancelled = &cancelled;
            let search = &search;
            match thread::Builder::new()
                .name(format!("mnemonic-worker-{}", index))
                .spawn_scoped(scope, move || {
                    let result = std::panic::catch_unwind(|| search(mnemonic1, cancelled))
                        .unwrap_or_else(|_| Err(anyhow!("A mnemonic worker panicked")));
                    // Stop peers on success, generation failure, or panic.
                    cancelled.store(true, Ordering::Relaxed);
                    if let Ok(None) = result {
                        return;
                    }
                    let _ = tx.send(result);
                }) {
                Ok(handle) => handles.push(handle),
                Err(error) => {
                    spawn_error = Some(error);
                    break;
                }
            }
        }
        drop(tx);
        let result = if let Some(error) = spawn_error {
            Err(anyhow!(error).context("Could not start mnemonic worker"))
        } else {
            rx.recv()
                .context("Mnemonic workers stopped without a result")
                .and_then(|result| result)
                .and_then(|phrase| phrase.context("Mnemonic search was cancelled"))
        };
        cancelled.store(true, Ordering::Relaxed);
        let mut panicked = false;
        for handle in handles {
            if handle.join().is_err() {
                panicked = true;
            }
        }
        ensure!(!panicked, "A mnemonic worker panicked");
        result
    })
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
                .value_name("mnemonic")
                .case_insensitive(true)
                .help("Set one of 12 word mnemonic phrases (Default: generate random)"),
        )
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .value_name("threads")
                .case_insensitive(true)
                .help("Set number of worker threads (Default: number of CPUs)"),
        )
        .get_matches();

    let cpus = args
        .value_of("threads")
        .map(str::parse::<usize>)
        .transpose()
        .context("Thread count must be a positive integer")?
        .unwrap_or_else(num_cpus::get);
    ensure!(cpus > 0, "Number of threads must be greater than 0");

    let mnemonic1 = args.value_of("from-mnemonic");
    let mnemonic1: String = match mnemonic1 {
        Some(mnemonic1) => (Bip39Mnemonic::from_phrase(mnemonic1)?)
            .phrase()
            .to_string(),
        None => Bip39Mnemonic::generate12()?.phrase().to_string(),
    };
    ensure!(
        mnemonic1.split_whitespace().count() == 12,
        "User entered mnemonic seed phrase must be 12 words long"
    );
    let mnemonic2 = generate_compatible_mnemonic(&mnemonic1, cpus)?;
    let mnemonic3 = format!("{} {}", mnemonic1, mnemonic2);
    let mnemonic4 = format!("{} {}", mnemonic2, mnemonic1);

    println!(
        "Phrase1: {}\nPhrase2: {}\nPhrase1+Phrase2: {}\nPhrase2+Phrase1: {}",
        mnemonic1, mnemonic2, mnemonic3, mnemonic4
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PHRASE: &str =
        "tell candy asset phrase welcome property below tackle inch distance what fish";

    #[test]
    fn cancelled_search_returns_without_generating() {
        assert!(find_compatible_mnemonic(PHRASE, &AtomicBool::new(true))
            .unwrap()
            .is_none());
    }

    #[test]
    fn zero_workers_are_rejected() {
        assert!(generate_compatible_mnemonic(PHRASE, 0).is_err());
    }

    #[test]
    fn worker_errors_and_panics_are_returned() {
        assert!(
            run_search_workers(PHRASE, 4, |_, _| Err(anyhow!("generation failed")))
                .unwrap_err()
                .to_string()
                .contains("generation failed")
        );
        assert!(
            run_search_workers(PHRASE, 4, |_, _| panic!("test worker panic"))
                .unwrap_err()
                .to_string()
                .contains("worker panicked")
        );
    }

    #[test]
    fn parallel_search_returns_compatible_phrases() {
        for workers in [1, 4] {
            let finished = std::sync::atomic::AtomicUsize::new(0);
            let second = run_search_workers(PHRASE, workers, |_, _| {
                finished.fetch_add(1, Ordering::Relaxed);
                Ok(Some("space shuffle input same second ability worry allow stage utility october exact".to_string()))
            }).unwrap();
            assert_eq!(finished.load(Ordering::Relaxed), workers);
            assert_eq!(second.split_whitespace().count(), 12);
            Bip39Mnemonic::validate(&second).unwrap();
            Bip39Mnemonic::validate(&format!("{} {}", PHRASE, second)).unwrap();
            Bip39Mnemonic::validate(&format!("{} {}", second, PHRASE)).unwrap();
        }
    }
}
