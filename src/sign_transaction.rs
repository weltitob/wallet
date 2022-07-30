use std::str::FromStr;

use bitcoin::util::psbt::PartiallySignedTransaction as Psbt;

use bdk::{Wallet, SignOptions};
use bdk::database::MemoryDatabase;

pub fn sign_transaction() -> Result<(), bdk::Error> {
    let wallet = Wallet::new(
        "wpkh([c258d2e4/84h/1h/0h]tprv8griRPhA7342zfRyB6CqeKF8CJDXYu5pgnj1cjL1u2ngKcJha5jjTRimG82ABzJQ4MQe71CV54xfn25BbhCNfEGGJZnxvCDQCd6JkbvxW6h/0/*)",
        Some("wpkh([c258d2e4/84h/1h/0h]tprv8griRPhA7342zfRyB6CqeKF8CJDXYu5pgnj1cjL1u2ngKcJha5jjTRimG82ABzJQ4MQe71CV54xfn25BbhCNfEGGJZnxvCDQCd6JkbvxW6h/1/*)"),
        bitcoin::Network::Testnet,
        MemoryDatabase::default(),
    )?;

    let psbt = "...";
    let mut psbt = Psbt::from_str(psbt)?;

    let finalized = wallet.sign(&mut psbt, SignOptions::default())?;

    Ok(())
}