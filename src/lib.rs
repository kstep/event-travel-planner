#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[macro_use]
#[cfg(not(target_arch = "wasm32"))]
extern crate diesel;

#[macro_use]
#[cfg(target_arch = "wasm32")]
extern crate yew_router;

mod common;

#[cfg(not(target_arch = "wasm32"))]
pub mod backend;

#[cfg(target_arch = "wasm32")]
pub mod frontend;
