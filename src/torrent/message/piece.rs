pub struct Piece {
    pub location: super::location::Location,
    pub piece: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Location(super::location::Error),
}

pub fn r#try(input: &[u8]) -> Result<Piece, Error> {
    let (input, location) = super::location::r#try(input).map_err(Error::Location)?;
    Ok(Piece {
        location,
        piece: input.to_vec(),
    })
}

impl Piece {
    pub fn encode(&self) -> Vec<u8> {
        vec![self.location.encode(), self.piece.clone()].concat()
    }
}
