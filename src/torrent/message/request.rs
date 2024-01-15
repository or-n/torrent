use crate::decode;

pub struct Request {
    pub location: super::location::Location,
    pub length: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Location(super::location::Error),
    NoLength,
}

pub fn r#try(input: &[u8]) -> Result<Request, Error> {
    let (input, location) = super::location::r#try(input).map_err(Error::Location)?;
    let (_, length) = decode::u32(input).ok_or(Error::NoLength)?;
    Ok(Request { location, length })
}

impl Request {
    pub fn _new(location: super::location::Location) -> Request {
        Request {
            location,
            length: 16 * 1024,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        vec![self.location.encode(), self.length.to_be_bytes().to_vec()].concat()
    }
}

impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.location, self.length)
    }
}
