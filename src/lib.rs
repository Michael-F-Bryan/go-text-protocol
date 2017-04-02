//! A library for parsing messages using the `Go Text Protocol`.
//!
//! If you want to check out the parser and how it works under the hood,
//! consult the `parser` module.

#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate regex;

#[macro_use]
mod macros;
pub mod parser;

pub use errors::*;
pub use parser::{RawCommand, Parser, parse};

custom_command!(#[doc = "My custom command"]
               enum MyCommand {
                   Foo,
               });



mod errors {
    error_chain!{

        foreign_links {
            Regex(::regex::Error) #[doc = "A regex error"];
        }

        errors {
            /// Whitespace was expected.
            NoWhitespace {}

            /// The string doesn't contain a command.
            NoCommand {}
        }
    }
}
