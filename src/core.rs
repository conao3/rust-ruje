use anyhow::Result;

pub fn read(input: &str) -> Result<&str> {
    Ok(input)
}

pub fn eval(input: &str) -> Result<&str> {
    Ok(input)
}

pub fn print(input: &str) -> Result<&str> {
    Ok(input)
}

pub fn rep(input: &str) -> Result<&str> {
    Ok(print(eval(read(input)?)?)?)
}
