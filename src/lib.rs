#![deny(dead_code, unused_imports)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
#[cfg(not(target_arch = "wasm32"))]
extern crate diesel;

mod common;

#[cfg(not(target_arch = "wasm32"))]
pub mod backend;
