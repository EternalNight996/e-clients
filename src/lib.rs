// #![allow(
//   clippy::cognitive_complexity,
//   clippy::large_enum_variant,
//   clippy::module_inception,
//   clippy::needless_doctest_main
// )]
// #![warn(
//   missing_debug_implementations,
//   missing_docs,
//   rust_2018_idioms,
//   // unreachable_pub
// )]
// #![deny(unused_must_use)]
// #![doc(test(
//   no_crate_inject,
//   attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
// ))]
// #![cfg_attr(docsrs, feature(doc_cfg))]
// #![cfg_attr(docsrs, allow(unused_attributes))]
// #![cfg_attr(loom, allow(dead_code, unreachable_pub))]

#[macro_use]
#[path = "./macros.rs"]
mod macros;
#[path = "./share.rs"]
pub mod share;
pub use share::*;
cfg_ftp! {
  pub mod ftp;
}

cfg_ssh! {
  pub mod ssh;
}

cfg_smb! {
  pub mod smb;
}

pub use remotefs;
