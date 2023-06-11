#![no_std]

#[cfg(not(feature = "binary-vendor"))]
mod contract;
mod proofs;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
