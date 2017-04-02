//! The parser module.
//!
//! This module contains all the machinery for turning a `Go Text Protocol`
//! line into a `RawCommand`, or a custom variant.
//!
//! # Examples
//!
//! You can use the `parse()` function to turn a line of text into a command.
//!
//! ```rust
//! use go_text_protocol::{RawCommand, parse};
//!
//! let line = "3 play black D5";
//! let parsed_command: RawCommand = parse(line).unwrap();
//!
//! assert_eq!(parsed_command.count, Some(3));
//! assert_eq!(parsed_command.name, "play");
//! assert_eq!(parsed_command.args, vec!["black", "D5"]);
//! ```
//!
//! You can even define your own strongly typed commands using the
//! `typed_command!()` macro (note that the variants are **not case sensitive**).
//!
//! ```rust
//! #[macro_use]
//! extern crate go_text_protocol;
//!
//! use go_text_protocol::parse;
//!
//! custom_command!(enum MyCommand {
//!   ShowBoard,
//!   Quit,
//! });
//!
//! fn main() {
//!   let line = "quit";
//!   let parsed_command: MyCommand = parse(line).unwrap();
//!
//!   assert_eq!(parsed_command, MyCommand::Quit);
//! }
//! ```

use std::str::FromStr;

use errors::*;
use regex::Regex;

/// Parse a single line and extract a command.
///
/// This function is generic, so you can get any type which can be coerced from
/// a `RawCommand` using `raw_command.into()`.
pub fn parse<C>(src: &str) -> Result<C>
    where C: From<RawCommand>
{
    let parser = Parser::new(src);
    parser.parse().map(|c| c.into())
}

/// A raw command containing the command name, an optional count, and its
/// arguments.
#[derive(Clone, PartialEq, Debug)]
pub struct RawCommand {
    /// An optional number attached to the command.
    pub count: Option<u32>,

    /// The name of the command itself.
    pub name: String,

    /// Zero or more arguments for the command.
    pub args: Vec<String>,
}

/// A line parser.
pub struct Parser {
    src: String,
    pointer: usize,
}


impl Parser {
    /// Create a new parser to parse a line.
    pub fn new(line: &str) -> Parser {
        Parser {
            src: line.to_string(),
            pointer: 0,
        }
    }

    /// Parse the source string into a `RawCommand`.
    pub fn parse(mut self) -> Result<RawCommand> {
        // Try to lex the provided string into its optional count, command, and
        // arguments.
        let (count, mut identifiers) = self.lex()
            .chain_err(|| "Failed to parse the line into tokens")?;

        // Make sure we got at least 1 identifier (i.e. the command name itself)
        if identifiers.len() < 1 {
            Err(ErrorKind::NoCommand.into())
        } else {
            let args = identifiers.split_off(1);

            Ok(RawCommand {
                   count: count,
                   name: identifiers[0].clone(),
                   args: args,
               })
        }
    }

    /// Does lexical analysis.
    ///
    /// This breaks the input string into an optional number (plus a space),
    /// followed by a number of space delimited strings (the command and args).
    fn lex(&mut self) -> Result<(Option<u32>, Vec<String>)> {
        let mut tokens = vec![];
        let mut count = None;

        if let Some(num) = self.read_number() {
            count = Some(num);
            self.skip_whitespace()?;
        }

        while let Some(next_token) = self.lex_identifier() {
            tokens.push(next_token);
            let _ = self.skip_whitespace();
        }

        Ok((count, tokens))
    }

    /// Try to read a number from the source string, moving the pointer if a
    /// match is found.
    fn read_number(&mut self) -> Option<u32> {
        let pattern = Regex::new(r"^\d+").unwrap();
        let substring = &self.src[self.pointer..];

        match pattern.find(substring) {
            None => None,
            Some(mat) => {
                let number_as_str = mat.as_str();
                self.pointer += number_as_str.len();
                let number = u32::from_str(number_as_str).unwrap();
                Some(number)
            }
        }
    }

    /// Move the pointer past any whitespace, returning an error if there
    /// wasn't any.
    fn skip_whitespace(&mut self) -> Result<()> {
        let pattern = Regex::new(r"^\s+").unwrap();
        let substring = &self.src[self.pointer..];

        let num_bytes_to_skip = match pattern.find(substring) {
            None => 0,
            Some(mat) => mat.as_str().len(),
        };

        self.pointer += num_bytes_to_skip;

        if num_bytes_to_skip == 0 {
            Err(ErrorKind::NoWhitespace.into())
        } else {
            Ok(())
        }
    }

    /// Try to match an identifier (any alphanumeric string).
    fn lex_identifier(&mut self) -> Option<String> {
        let pattern = Regex::new(r"^[\w\d]+").unwrap();
        let substring = &self.src[self.pointer..];

        match pattern.find(substring) {
            None => None,
            Some(mat) => {
                let token = mat.as_str().to_string();
                self.pointer += token.len();
                Some(token)
            }
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_number() {
        let src = "123";
        let mut lexer = Parser::new(src);
        let should_be = 123;

        assert_eq!(lexer.pointer, 0);
        let got = lexer.read_number();

        assert_eq!(got, Some(should_be));
        assert_eq!(lexer.pointer, 3);
    }

    #[test]
    fn lex_whitespace() {
        let src = "    ";
        let mut lexer = Parser::new(src);

        assert_eq!(lexer.pointer, 0);
        lexer.skip_whitespace().unwrap();
        assert_eq!(lexer.pointer, 4);
    }

    #[test]
    fn lex_identifier() {
        let src = "asd".to_string();
        let mut lexer = Parser::new(src.as_str());
        let should_be = src;

        assert_eq!(lexer.pointer, 0);
        let got = lexer.lex_identifier();
        assert_eq!(lexer.pointer, 3);

        assert_eq!(got, Some(should_be));
    }

    #[test]
    fn lex_a_string() {
        let src = "123 hello";
        let mut lexer = Parser::new(src);
        let count_should_be = Some(123);
        let identifiers_should_be = vec!["hello".to_string()];

        let (count, identifiers) = lexer.lex().unwrap();

        assert_eq!(count, count_should_be);
        assert_eq!(identifiers, identifiers_should_be);
    }

    #[test]
    fn parse_a_command() {
        let src = "123 hello arg1 arg2 arg3";
        let parser = Parser::new(src);
        let should_be = RawCommand {
            count: Some(123),
            name: "hello".to_string(),
            args: vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()],
        };

        let got = parser.parse().unwrap();

        assert_eq!(got, should_be);
    }


    #[test]
    fn test_parse_function() {
        let src = "123 hello arg1 arg2 arg3";
        let should_be = RawCommand {
            count: Some(123),
            name: "hello".to_string(),
            args: vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()],
        };

        let got: RawCommand = parse(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_into_any_type() {
        // Here we define some custom type which can be converted from a
        // RawCommand using into().

        #[derive(Debug, PartialEq)]
        struct QuickCmd(Option<u32>, String, Vec<String>);

        impl From<RawCommand> for QuickCmd {
            fn from(other: RawCommand) -> Self {
                QuickCmd(other.count, other.name, other.args)
            }
        }

        let src = "123 hello arg1 arg2 arg3";
        let should_be = QuickCmd(Some(123),
                                 "hello".to_string(),
                                 vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()]);

        let got: QuickCmd = parse(src).unwrap();

        assert_eq!(got, should_be);
    }
}
