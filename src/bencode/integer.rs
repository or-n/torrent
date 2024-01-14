use crate::decode;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    Decode(decode::Error),
    ParseInt(ParseIntError),
    LeadingZeroes,
    NegativeZero,
}

pub fn r#try(input: &[u8]) -> Option<Result<(&[u8], i64), Error>> {
    let input = decode::char(input, b'i').ok()?;
    Some(end(input))
}

fn end(input: &[u8]) -> Result<(&[u8], i64), Error> {
    let (input, sign) = match decode::char(input, b'-') {
        Ok(input) => (input, -1),
        _ => (input, 1),
    };
    let (input, digits) = decode::till(input, b'e').map_err(Error::Decode)?;
    if digits.iter().take_while(|&c| c == &b'0').count() > 0 {
        if digits.len() == 1 {
            if sign == 1 {
                Ok((input, 0))
            } else {
                Err(Error::NegativeZero)
            }
        } else {
            Err(Error::LeadingZeroes)
        }
    } else {
        let n = String::from_utf8_lossy(digits)
            .parse::<i64>()
            .map_err(Error::ParseInt)?;
        Ok((input, n * sign))
    }
}
