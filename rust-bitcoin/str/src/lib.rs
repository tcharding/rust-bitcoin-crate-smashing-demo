//! bitcoin string encoding crate.
//!

// Re-export sub-crates and modules so users do not need to care about the crate/directory structure
// of bitcoin-str.

pub extern crate base58;
pub extern crate hex;
pub extern crate bech32;

/// Dummy values demo'ing re-exports of types from sub-crates.
pub use base58::BASE58;
pub use base64::BASE64;
pub use bech32::u5;
pub use hex::HEX;

// base64 module, demonstrates that we can mix re-exporting crates with code/modules local to the str crate.
// QUESTION: Is there any benefit to this versus having base64 being a separate crate?
pub mod base64 {
    pub const BASE64: u32 = 1;
}
