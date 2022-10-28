use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
pub enum Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

impl FromStr for Suit {
    type Err = SuitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(Self::HEARTS),
            "D" => Ok(Self::DIAMONDS),
            "C" => Ok(Self::CLUBS),
            "S" => Ok(Self::SPADES),
            _ => Err(SuitError),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Suit::HEARTS => write!(f, "Hearts"),
            Suit::DIAMONDS => write!(f, "Diamonds"),
            Suit::CLUBS => write!(f, "Clubs"),
            Suit::SPADES => write!(f, "Spades"),
        }
    }
}

pub struct SuitError;
