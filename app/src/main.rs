// Demo use for an app that cares about a bunch of string encodings, ergonomical to use like this.
use bitcoin::str::{BASE58, BASE64, HEX, u5};

// Demo use for an app that cares only about hex, ergonomic to use like this.
use bitcoin::hex;

// Demo use of `hashes` crate.
use bitcoin::hashes::HASHES;

fn main() {
    do_some_hex_stuff();
    do_some_str_stuff();
    do_some_hashes_stuff();
}

fn do_some_hex_stuff() {
    let h = hex::HEX;           // More realistically this would be `hex::encode()`.
    println!("Did some hex stuff: {}", h);
}

fn do_some_str_stuff() {
    let base58 = BASE58;
    let base64 = BASE64;
    let hashes = HASHES;
    let hex = HEX;
    let u = u5::try_from_u8(1).expect("failed to create u5");

    println!("Did some str stuff: {} {} {} {} {:?}", base58, base64, hashes, hex, u);
}

fn do_some_hashes_stuff() {
    let hashes = HASHES;

    println!("Did some hashes stuff: {}", hashes);
}
