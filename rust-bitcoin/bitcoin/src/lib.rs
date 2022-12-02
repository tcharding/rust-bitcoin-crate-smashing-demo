// The main bitcoin crate.

// Re-export crates with a more terse name so users of rust-bitcoin can do `use bitcoin::str::Foo`.
pub extern crate bitcoin_str as str;
// Re-export sub-crates so users do not need to care about the crate/directory structure of rust-bitcoin.
pub extern crate hashes;

// Re-export these?
pub use bitcoin_str::{base58, base64, bech32, hex};

// These are not re-exported, users required to qualify with `str`: `use bitcoin::str::base58`?
use bitcoin_str::{ BASE58, BASE64, HEX};
// Perhaps some things from `str` should be re-exported?
pub use bitcoin_str::u5;

pub fn use_dependency() {
    let _ = BASE58;
    let _ = BASE64;
    let _ = HEX;

    let _ = base58::BASE58;
    let _ = base64::BASE64;
    let _ = hex::HEX;

    let _ = u5::try_from_u8(1).expect("failed to create u5");
    let _ = bech32::u5::try_from_u8(1).expect("failed to create u5");

    let _ = hashes::HASHES;
}
