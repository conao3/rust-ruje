use std::fmt;

use crate::types::RujeAtom;

#[derive(Debug, PartialEq)]
pub enum RujeExp {
    Atom(RujeAtom),
    List(Vec<RujeExp>),
    Vector(Vec<RujeExp>),
    Map(Vec<(RujeExp, RujeExp)>),
    Set(Vec<RujeExp>),
}

impl fmt::Display for RujeExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RujeExp::Atom(e) => write!(f, "{}", e),
            RujeExp::List(e) => {
                write!(f, "(")?;
                write!(
                    f,
                    "{}",
                    e.iter()
                        .map(|exp| exp.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
                write!(f, ")")
            }
            RujeExp::Vector(e) => {
                write!(f, "[")?;
                write!(
                    f,
                    "{}",
                    e.iter()
                        .map(|exp| exp.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
                write!(f, "]")
            }
            RujeExp::Map(e) => {
                write!(f, "{{")?;
                write!(
                    f,
                    "{}",
                    e.iter()
                        .map(|(k, v)| format!("{} {}", k, v))
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
                write!(f, "}}")
            }
            RujeExp::Set(e) => {
                write!(f, "#{{")?;
                write!(
                    f,
                    "{}",
                    e.iter()
                        .map(|exp| exp.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
                write!(f, "}}")
            }
        }
    }
}

impl From<RujeAtom> for RujeExp {
    fn from(value: RujeAtom) -> Self {
        RujeExp::Atom(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RujeAtom;

    #[test]
    fn test_atom() {
        let atom = RujeAtom::new_symbol("test");
        assert_eq!(atom.to_string(), "test");
    }

    #[test]
    fn test_exp() {
        let exp = RujeExp::Atom(RujeAtom::Symbol("test".to_string()));
        assert_eq!(exp.to_string(), "test");

        let exp = RujeExp::List(vec![
            RujeExp::Atom(RujeAtom::Symbol("test".to_string())),
            RujeExp::Atom(RujeAtom::Symbol("test".to_string())),
        ]);
        assert_eq!(exp.to_string(), "(test test)");

        let exp = RujeExp::Vector(vec![
            RujeExp::Atom(RujeAtom::Symbol("test".to_string())),
            RujeExp::Atom(RujeAtom::Symbol("test".to_string())),
        ]);
        assert_eq!(exp.to_string(), "[test test]");

        let exp = RujeExp::Map(vec![
            (
                RujeExp::Atom(RujeAtom::Symbol("test".to_string())),
                RujeExp::Atom(RujeAtom::Symbol("test".to_string())),
            ),
            (
                RujeExp::Atom(RujeAtom::Symbol("test".to_string())),
                RujeExp::Atom(RujeAtom::Symbol("test".to_string())),
            ),
        ]);
        assert_eq!(exp.to_string(), "{test test test test}");

        let exp = RujeExp::Set(vec![
            RujeExp::Atom(RujeAtom::Symbol("test".to_string())),
            RujeExp::Atom(RujeAtom::Symbol("test".to_string())),
        ]);
        assert_eq!(exp.to_string(), "#{test test}");
    }
}
