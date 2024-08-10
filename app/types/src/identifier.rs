use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Identifier<T = i32>(pub T);

impl FromStr for Identifier {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match i32::from_str(value) {
            Ok(id) => Ok(Identifier { 0: id }),
            Err(_) => Err(())
        }
    }
}

impl From<Identifier> for i32 {
    fn from(value: Identifier) -> Self {
        value.0
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.0.to_string()
    }
}

impl From<i32> for Identifier {
    fn from(value: i32) -> Self {
        Identifier { 0: value }
    }
}