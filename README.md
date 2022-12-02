# rust-bitcoin-crate-smashing-demo

Repo used to demonstrate potential crate/directory structure for rust-bitcoin while crate smashing

## Directory list

- `app`: A dummy application that uses `rust-bitcoin` library.
- `rust-bitcoin`: Represents the `rust-bitcoin` repository.

## Status

Demo currently includes:

- rust-bitcoin
  - bitcoin
  - hashes
  - internals
  - str
    - hex    (sub-crate)
    - base58 (sub-crate)
    - base64 (local code)
    - bech32 (re-export)
