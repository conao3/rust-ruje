use crate::types::{RujeAtom, RujeExp};
use anyhow::anyhow;
use anyhow::Result;

use regex::Regex;
use std::sync::LazyLock;

static INT_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([+-]?[0-9]+)(?:[ ();]|$)").unwrap());
static FLOAT_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([+-]?[0-9]*\.[0-9]+)(?:[ ();]|$)").unwrap());
static SYMBOL_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[^ ();]+").unwrap());

pub struct Reader<'a> {
    input: &'a str,
}

impl Reader<'_> {
    pub fn new(input: &str) -> Reader<'_> {
        Reader { input }
    }

    fn skip_whitespace(&mut self) {
        self.input = self.input.trim_start();
    }

    fn read_atom(&mut self) -> Result<RujeExp> {
        self.skip_whitespace();

        if let Some(m) = INT_PATTERN.find(self.input) {
            let value = m.as_str().parse::<i64>().unwrap();
            self.input = &self.input[m.end()..];
            return Ok(RujeAtom::Int(value).into());
        }

        if let Some(m) = FLOAT_PATTERN.find(self.input) {
            let value = m.as_str().parse::<f64>().unwrap();
            self.input = &self.input[m.end()..];
            return Ok(RujeAtom::Float(value).into());
        }

        if let Some(m) = SYMBOL_PATTERN.find(self.input) {
            let value = m.as_str().to_string();
            self.input = &self.input[m.end()..];
            return Ok(RujeAtom::Symbol(value).into());
        }

        Err(anyhow::anyhow!("Invalid input"))
    }

    pub fn read(&mut self) -> Result<RujeExp> {
        self.skip_whitespace();

        let c = self
            .input
            .chars()
            .next()
            .ok_or(anyhow!("Unexpected end of input"))?;

        match c {
            _ => self.read_atom()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RujeAtom;

    #[test]
    fn test_read_atom() {
        let mut reader = Reader::new("123");
        let atom = reader.read_atom().unwrap();
        assert_eq!(atom, RujeAtom::Int(123).into());

        let mut reader = Reader::new("123.456");
        let atom = reader.read_atom().unwrap();
        assert_eq!(atom, RujeAtom::Float(123.456).into());

        let mut reader = Reader::new("abc");
        let atom = reader.read_atom().unwrap();
        assert_eq!(atom, RujeAtom::Symbol("abc".to_string()).into());
    }
}
