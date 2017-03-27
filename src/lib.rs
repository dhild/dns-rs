//! A Rust library for parsing resource records.
#![doc(html_root_url = "https://dhild.github.io/martin/")]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate nom;

mod errors;
mod names;
mod rr;
mod header;
mod question;
mod message;
mod parse;

pub use header::{Opcode, Rcode};
pub use message::Message;
pub use names::Name;
pub use question::{Question, QType};
pub use rr::{Type, Class, ResourceRecord};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
