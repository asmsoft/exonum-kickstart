#![cfg_attr(feature="cargo-clippy", allow(zero_prefixed_literal))]

extern crate exonum;
#[macro_use]
extern crate exonum_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

extern crate protobuf;

pub mod api;
pub mod blockchain;
mod service;
mod proto;

pub use service::{project_nameService, SERVICE_NAME, SERVICE_ID};