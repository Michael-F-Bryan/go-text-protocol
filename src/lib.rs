#[macro_use]
extern crate error_chain;
extern crate regex;

#[macro_use]
mod macros;
mod parser;

pub use errors::*;
pub use parser::{RawCommand, Parser};

typed_command!(#[doc = "My custom command"]
               enum MyCommand {
                   Foo
               });



pub mod errors {
    error_chain!{

        foreign_links {
            Regex(::regex::Error);
        }

        errors {
            NoWhitespace {}
            NoCommand {}
        }
    }
}
