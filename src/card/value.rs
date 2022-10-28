use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

pub enum Value {
    ACE,
    KING,
    QUEEN,
    JACK,
    TEN,
    NINE,
    EIGHT,
    SEVEN,
    SIX,
    FIVE,
    FOUR,
    THREE,
    TWO,
    ONE,
}

impl FromStr for Value {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::ACE),
            "K" => Ok(Self::KING),
            "Q" => Ok(Self::QUEEN),
            "J" => Ok(Self::JACK),
            "10" => Ok(Self::TEN),
            "9" => Ok(Self::NINE),
            "8" => Ok(Self::EIGHT),
            "7" => Ok(Self::SEVEN),
            "6" => Ok(Self::SIX),
            "5" => Ok(Self::FIVE),
            "4" => Ok(Self::FOUR),
            "3" => Ok(Self::THREE),
            "2" => Ok(Self::TWO),
            "1" => Ok(Self::ONE),
            _ => Err(ValueError),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Value::ACE => write!(f, "Ace"),
            Value::KING => write!(f, "King"),
            Value::QUEEN => write!(f, "Queen"),
            Value::JACK => write!(f, "Jack"),
            Value::TEN => write!(f, "10"),
            Value::NINE => write!(f, "9"),
            Value::EIGHT => write!(f, "8"),
            Value::SEVEN => write!(f, "7"),
            Value::SIX => write!(f, "6"),
            Value::FIVE => write!(f, "5"),
            Value::FOUR => write!(f, "4"),
            Value::THREE => write!(f, "3"),
            Value::TWO => write!(f, "2"),
            Value::ONE => write!(f, "1"),
        }
    }
}

pub struct ValueError;
