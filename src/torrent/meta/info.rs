use crate::bencode;
use crate::util;

pub struct Info {
    pub length: i64,
    pub name: String,
    pub piece_length: i64,
    pub pieces: Vec<[u8; 20]>,
}

use util::bencode::{integer_field, string_field, FieldError};

#[derive(Debug)]
pub enum Error {
    NotDictionary,
    Length(FieldError),
    Name(FieldError),
    PieceLength(FieldError),
    Pieces(FieldError),
}

pub fn extract(item: &bencode::Item) -> Result<Info, Error> {
    let d = util::bencode::dictionary(item).ok_or(Error::NotDictionary)?;
    Ok(Info {
        length: integer_field("length", d).map_err(Error::Length)?,
        name: format!("{:?}", string_field("name", d).map_err(Error::Name)?),
        piece_length: integer_field("piece length", d).map_err(Error::PieceLength)?,
        pieces: string_field("pieces", d)
            .map_err(Error::Pieces)?
            .bytes
            .chunks(20)
            .map(|x| x.try_into())
            .flatten()
            .collect(),
    })
}
