use crate::decode;

pub struct Location {
    pub index: u32,
    pub begin: u32,
}

#[derive(Debug)]
pub enum Error {
    NoIndex,
    NoBegin,
}

pub fn r#try(input: &[u8]) -> Result<(&[u8], Location), Error> {
    let (input, index) = decode::u32(input).ok_or(Error::NoIndex)?;
    let (input, begin) = decode::u32(input).ok_or(Error::NoBegin)?;
    Ok((input, Location { index, begin }))
}

impl Location {
    pub fn encode(&self) -> Vec<u8> {
        vec![
            self.index.to_be_bytes().to_vec(),
            self.begin.to_be_bytes().to_vec(),
        ]
        .concat()
    }
}

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.index, self.begin)
    }
}
