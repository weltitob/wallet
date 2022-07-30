/*
The following code shows how to generate a random mnemonic,
the extended (and deterministic) keys from that mnemonic and
finally the descriptors from the extended private keys

the generalization is achieved thanks to descriptors (opens new window),
that are now slowly starting to see adoption in a few other Bitcoin projects as well.
*/

use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::keys::{DerivableKey, GeneratableKey, GeneratedKey, ExtendedKey, bip39::{Mnemonic, WordCount, Language}};
use bdk::template::Bip84;
use bdk::{miniscript, Wallet, KeychainKind};

fn main() {

    let network = Network::Testnet; // Or this can be Network::Bitcoin, Network::Signet or Network::Regtest

    // Generate fresh mnemonic
    let mnemonic: GeneratedKey<_, miniscript::Segwitv0> = Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();

    // Convert mnemonic to string
    let mnemonic_words = mnemonic.to_string();
    println!("These are the recovery words for the wallet: {}", mnemonic_words);

    // Parse a mnemonic
    let mnemonic  = Mnemonic::parse(&mnemonic_words).unwrap();

    // Generate the extended key
    //maybe publickey for wallet?
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    // Get xprv from the extended key
    //maybe privatekey for wallet?
    let xprv = xkey.into_xprv(network).unwrap();

    // Create a BDK wallet structure using BIP 84 descriptor ("m/84h/1h/0h/0" and "m/84h/1h/0h/1")
    //is offline wallet but we want to crreate others?
    let wallet = Wallet::new(
        Bip84(xprv, KeychainKind::External),
        Some(Bip84(xprv, KeychainKind::Internal)),
        network,
        MemoryDatabase::default(),
    )
    .unwrap();

    //find out what unwrap does (entpacken / auspacken)

    println!("mnemonic: {}\n\nrecv desc (pub key): {:#?}\n\nchng desc (pub key): {:#?}",
        mnemonic_words,
        wallet.get_descriptor_for_keychain(KeychainKind::External).to_string(),
        wallet.get_descriptor_for_keychain(KeychainKind::Internal).to_string());
}