use anyhow::Result;

use crate::{reader, types::RujeExp};

pub fn read(input: &str) -> Result<RujeExp> {
    let mut reader = reader::Reader::new(input);
    Ok(reader.read()?)
}

pub fn eval(input: RujeExp) -> Result<RujeExp> {
    Ok(input)
}

pub fn print(input: RujeExp) -> Result<String> {
    Ok(input.to_string())
}

pub fn rep(input: &str) -> Result<String> {
    Ok(print(eval(read(input)?)?)?)
}
