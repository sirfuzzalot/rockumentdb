use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct KeyNotFoundError;

impl fmt::Display for KeyNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyNotFoundError")
    }
}

impl Error for KeyNotFoundError {}

#[derive(Debug)]
pub struct RangeError;

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RangeError")
    }
}

impl Error for RangeError {}
