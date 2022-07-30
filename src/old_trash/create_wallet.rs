use bdk::{Wallet};
use bdk::database::MemoryDatabase;
use bdk::wallet::AddressIndex::New;

pub fn create_wallet() -> Result<(), bdk::Error> {
    let wallet = Wallet::new(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
            bitcoin::Network::Testnet,
            MemoryDatabase::default(),
        )?;
    
        println!("Address #0: {}", wallet.get_address(New)?);
        println!("Address #1: {}", wallet.get_address(New)?);
        println!("Address #2: {}", wallet.get_address(New)?);
        println!("Address #3: {}", wallet.get_address(New)?);
    
        Ok(())
}