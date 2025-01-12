use crate::types::{RujeAtom, RujeExp};
use anyhow::bail;
use anyhow::Result;

use regex::Regex;
use std::sync::LazyLock;

static INT_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([+-]?[0-9]+)(?:[ ();\[\]{}]|$)").unwrap());
static FLOAT_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([+-]?[0-9]*\.[0-9]+)(?:[ ();\[\]{}]|$)").unwrap());
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

        if let Some(m) = INT_PATTERN.captures(self.input) {
            let s = m.get(1).unwrap().as_str();
            let value = s.parse::<i64>().unwrap();
            self.input = &self.input[s.len()..];
            return Ok(RujeAtom::Int(value).into());
        }

        if let Some(m) = FLOAT_PATTERN.captures(self.input) {
            let s = m.get(1).unwrap().as_str();
            let value = s.parse::<f64>().unwrap();
            self.input = &self.input[s.len()..];
            return Ok(RujeAtom::Float(value).into());
        }

        if let Some(m) = SYMBOL_PATTERN.captures(self.input) {
            let s = m.get(0).unwrap().as_str();
            let value = s.to_string();
            self.input = &self.input[s.len()..];
            return Ok(RujeAtom::Symbol(value).into());
        }
        Err(anyhow::anyhow!("Invalid input"))
    }

    fn read_list(&mut self) -> Result<RujeExp> {
        self.input = &self.input[1..]; // skip '('

        let c = self.input.chars().next();

        match c {
            None => bail!("Unexpected end of input"),
            Some(')') => {
                self.input = &self.input[1..]; // skip ')'
                return Ok(RujeExp::List(vec![]));
            }
            Some(_) => {
                let mut list = vec![];
                loop {
                    let exp = self.read()?;
                    list.push(exp);
                    self.skip_whitespace();
                    match self.input.chars().next() {
                        None => bail!("Unexpected end of input"),
                        Some(')') => {
                            self.input = &self.input[1..]; // skip ')'
                            return Ok(RujeExp::List(list));
                        }
                        Some(_) => continue,
                    }
                }
            }
        }
    }

    fn read_vector(&mut self) -> Result<RujeExp> {
        self.input = &self.input[1..]; // skip '['

        let c = self.input.chars().next();

        match c {
            None => bail!("Unexpected end of input"),
            Some(']') => {
                self.input = &self.input[1..]; // skip ']'
                return Ok(RujeExp::Vector(vec![]));
            }
            Some(_) => {
                let mut list = vec![];
                println!("{}", self.input);
                loop {
                    let exp = self.read()?;
                    list.push(exp);
                    self.skip_whitespace();
                    println!("{}", self.input);
                    match self.input.chars().next() {
                        None => bail!("Unexpected end of input"),
                        Some(']') => {
                            self.input = &self.input[1..]; // skip ']'
                            return Ok(RujeExp::Vector(list));
                        }
                        Some(_) => continue,
                    }
                }
            }
        }
    }

    fn read_map(&mut self) -> Result<RujeExp> {
        self.input = &self.input[1..]; // skip '{'

        let c = self.input.chars().next();

        match c {
            None => bail!("Unexpected end of input"),
            Some('}') => {
                self.input = &self.input[1..]; // skip '}'
                return Ok(RujeExp::Map(vec![]));
            }
            Some(_) => {
                let mut list = vec![];
                loop {
                    let key = self.read()?;
                    let val = self.read()?;
                    list.push((key, val));
                    self.skip_whitespace();
                    match self.input.chars().next() {
                        None => bail!("Unexpected end of input"),
                        Some('}') => {
                            self.input = &self.input[1..]; // skip '}'
                            return Ok(RujeExp::Map(list));
                        }
                        Some(_) => continue,
                    }
                }
            }
        }
    }

    fn read_set(&mut self) -> Result<RujeExp> {
        self.input = &self.input[1..]; // skip '{'

        let c = self.input.chars().next();

        match c {
            None => bail!("Unexpected end of input"),
            Some('}') => {
                self.input = &self.input[1..]; // skip '}'
                return Ok(RujeExp::Set(vec![]));
            }
            Some(_) => {
                let mut list = vec![];
                loop {
                    let exp = self.read()?;
                    list.push(exp);
                    self.skip_whitespace();
                    match self.input.chars().next() {
                        None => bail!("Unexpected end of input"),
                        Some('}') => {
                            self.input = &self.input[1..]; // skip '}'
                            return Ok(RujeExp::Set(list));
                        }
                        Some(_) => continue,
                    }
                }
            }
        }
    }

    pub fn read(&mut self) -> Result<RujeExp> {
        self.skip_whitespace();

        let c = self.input.chars().next();

        match c {
            None => bail!("Unexpected end of input"),
            Some('(') => self.read_list(),
            Some('[') => self.read_vector(),
            Some('{') => self.read_map(),
            Some('#') => {
                self.input = &self.input[1..]; // skip '#'
                let c = self.input.chars().next();
                match c {
                    None => bail!("Unexpected end of input"),
                    Some('{') => self.read_set(),
                    _ => bail!("Unknown reader macro char"),
                }
            }
            Some(')') => bail!("Unexpected ')'"),
            Some(']') => bail!("Unexpected ']'"),
            Some('}') => bail!("Unexpected '}}'"),
            Some(_) => self.read_atom(),
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

    #[test]
    fn test_read_list() {
        let mut reader = Reader::new("()");
        let list = reader.read().unwrap();
        assert_eq!(list, RujeExp::List(vec![]));

        let mut reader = Reader::new("(123 456)");
        let list = reader.read().unwrap();
        assert_eq!(
            list,
            RujeExp::List(vec![RujeAtom::Int(123).into(), RujeAtom::Int(456).into()])
        );
    }

    #[test]
    fn test_read_vector() {
        let mut reader = Reader::new("[]");
        let list = reader.read().unwrap();
        assert_eq!(list, RujeExp::Vector(vec![]));

        let mut reader = Reader::new("[123 456]");
        let list = reader.read().unwrap();
        assert_eq!(
            list,
            RujeExp::Vector(vec![RujeAtom::Int(123).into(), RujeAtom::Int(456).into()])
        );
    }

    #[test]
    fn test_read_map() {
        let mut reader = Reader::new("{}");
        let list = reader.read().unwrap();
        assert_eq!(list, RujeExp::Map(vec![]));

        let mut reader = Reader::new("{a 123}");
        let list = reader.read().unwrap();
        assert_eq!(
            list,
            RujeExp::Map(vec![(
                RujeAtom::Symbol("a".to_string()).into(),
                RujeAtom::Int(123).into()
            )])
        )
    }

    #[test]
    fn test_read_set() {
        let mut reader = Reader::new("#{}");
        let list = reader.read().unwrap();
        assert_eq!(list, RujeExp::Set(vec![]));

        let mut reader = Reader::new("#{a 123}");
        let list = reader.read().unwrap();
        assert_eq!(
            list,
            RujeExp::Set(vec![
                RujeAtom::Symbol("a".to_string()).into(),
                RujeAtom::Int(123).into(),
            ])
        )
    }
}
