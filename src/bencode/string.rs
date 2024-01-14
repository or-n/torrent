use crate::decode;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Bytes {
    pub bytes: Vec<u8>,
}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bytes.len() > 100 {
            return write!(f, "... ({} bytes)", self.bytes.len());
        }
        for &i in &self.bytes {
            let c = i as char;
            match c {
                '\\' | '\"' => write!(f, "\\{}", c)?,
                '\x20'..='\x7E' => write!(f, "{}", c)?,
                _ => write!(f, "\\u{:04X}", i)?,
            }
        }
        Ok(())
    }
}

impl serde::Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{:?}", self).as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ParseInt(std::num::ParseIntError),
    NotEnough,
}

pub fn r#try(input: &[u8]) -> Option<Result<(&[u8], Bytes), Error>> {
    let (input, digits) = decode::till(input, b':').ok()?;
    Some(end(input, digits))
}

fn end<'a>(input: &'a [u8], digits: &'a [u8]) -> Result<(&'a [u8], Bytes), Error> {
    let n = String::from_utf8_lossy(digits)
        .parse::<usize>()
        .map_err(Error::ParseInt)?;
    if input.len() < n {
        return Err(Error::NotEnough);
    }
    let (bytes, input) = input.split_at(n);
    let bytes = bytes.into();
    Ok((input, Bytes { bytes }))
}
