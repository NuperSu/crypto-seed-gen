# Crypto Seed Generator

This is a utility to generate cool seed phrases of 12 and 24 words.
This way, you only need to remember just two 12 words seed phrases but be able to use 4 different valid seed phrase .
We support both [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) for generating the keys from the mnemonic (see details further below).

Generating a wallet from a seed phrase is a good way to secure your funds. You can, for example, print out the seed phrase (or etch it into metal cards for extra durability)
and store it offline. With this seed phrase (and the chosen password, if any), you can always restore access to your funds if the hard drive with your
crypto money happens to die. Or you carry it with you to get access to your funds from somewhere else.

## Usage

#### 1. Generate seed phrase

```
$ crypto-seed-gen
Phrase1: tell candy asset phrase welcome property below tackle inch distance what fish
Phrase2: space shuffle input same second ability worry allow stage utility october exact
Phrase1+Phrase2: tell candy asset phrase welcome property below tackle inch distance what fish space shuffle input same second ability worry allow stage utility october exact
Phrase2+Phrase1: space shuffle input same second ability worry allow stage utility october exact tell candy asset phrase welcome property below tackle inch distance what fish
```

The "Phase1" and "Phrase2" are seed phrases you need to remember or print.

#### 2. Generate a Phrase2 with the same Phrase1 seed phrase

```
$ crypto-seed-gen -m "tell candy asset phrase welcome property below tackle inch distance what fish" 
Phrase1: tell candy asset phrase welcome property below tackle inch distance what fish
Phrase2: tray history awesome typical dilemma adapt note rib rent harbor lonely emotion
Phrase1+Phrase2: tell candy asset phrase welcome property below tackle inch distance what fish tray history awesome typical dilemma adapt note rib rent harbor lonely emotion
Phrase2+Phrase1: tray history awesome typical dilemma adapt note rib rent harbor lonely emotion tell candy asset phrase welcome property below tackle inch distance what fish
```

It generated such Phrase2 that all the outputed seed phrases are valid.

## Installation

#### 1. Install cargo (package manager for the rust programming language)

You can use [this one-step install command](https://www.rust-lang.org/tools/install).

You might have to call this afterwards, or alternatively just restart your bash session:
```
$ source $HOME/.cargo/env
```

Also make sure, you have openssl and a linker installed, for example by running the following:
```
$ sudo apt install libssl-dev pkg-config gcc
```

#### 2. Install crypto-seed-gen
```
$ cargo install --path .
```

#### 3. You can now run the program with
```
$ crypto-seed-gen
```

## How keys are derived

This tool uses [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) for the mnemonic and bruteforces through them to get valid 24 words seed phrase "Phrase1+Phrase2" and "Phrase2+Phrase1"
It mainly relies on [crypto-wallet-gen made by smessmer](https://github.com/smessmer/crypto-wallet-gen) for its basic structure.
