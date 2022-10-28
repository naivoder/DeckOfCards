use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use super::suit::{Suit, SuitError};
use super::value::{Value, ValueError};

const DEBUG: u8 = 0;

pub struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    pub fn new(value: &str, suit: &str) -> Result<Self, CardError> {
        if DEBUG > 0 {
            println!("Creating card: {} of {}", value, suit);
        }
        let value: Value = value.parse()?;
        let suit: Suit = suit.parse()?;
        return Ok(Self { value, suit });
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

pub enum CardError {
    InvalidSuit,
    InvalidValue,
}

impl CardError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidSuit => "Invalid Suit",
            Self::InvalidValue => "Invalid Value",
        }
    }
}

impl From<SuitError> for CardError {
    fn from(_: SuitError) -> Self {
        Self::InvalidSuit
    }
}

impl From<ValueError> for CardError {
    fn from(_: ValueError) -> Self {
        Self::InvalidValue
    }
}

impl Display for CardError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for CardError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for CardError {}
