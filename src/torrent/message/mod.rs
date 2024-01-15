use crate::decode;

pub mod location;
pub mod piece;
pub mod request;

pub enum Message {
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have(u32),
    Bitfield(Vec<u8>),
    Request(request::Request),
    Piece(piece::Piece),
    Cancel(request::Request),
}

pub fn show_bitfield(bitfield: &Vec<u8>) -> String {
    bitfield
        .iter()
        .flat_map(|&byte| (0..8).rev().map(move |i| (byte >> i) & 1))
        .map(|bit| format!("{}", bit))
        .collect()
}

use Message::*;

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Choke => write!(f, "choke"),
            Unchoke => write!(f, "unchoke"),
            Interested => write!(f, "interested"),
            NotInterested => write!(f, "not interested"),
            Have(index) => write!(f, "have {}", index),
            Bitfield(x) => write!(f, "bitfield {}", show_bitfield(x)),
            Request(x) => write!(f, "request {:?}", x),
            Piece(x) => write!(f, "piece {:?}", x.location),
            Cancel(x) => write!(f, "cancel {:?}", x),
        }
    }
}

impl Message {
    pub fn encode(&self) -> Vec<u8> {
        let number = |n: u32| n.to_be_bytes().to_vec();
        let encode = |n, bytes: Vec<u8>| vec![number(bytes.len() as u32 + 1), vec![n], bytes];
        match self {
            Choke => vec![number(0), vec![0]],
            Unchoke => vec![number(0), vec![1]],
            Interested => vec![number(0), vec![2]],
            NotInterested => vec![number(0), vec![3]],
            Have(x) => encode(4, number(*x as u32)),
            Bitfield(bytes) => encode(5, bytes.clone()),
            Request(request) => encode(6, request.encode()),
            Piece(piece) => encode(7, piece.encode()),
            Cancel(request) => encode(8, request.encode()),
        }
        .concat()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NoLength,
    NotEnoughBytes,
    NotValidType,
    Have,
    Request(request::Error),
    Piece(piece::Error),
    Cancel(request::Error),
}

pub enum Action {
    Message(Message),
    KeepAlive,
}

pub fn r#try(input: &[u8]) -> Result<(&[u8], Action), Error> {
    use Action::*;
    let (input, n) = decode::u32(input).ok_or(Error::NoLength)?;
    if n == 0 {
        return Ok((input, KeepAlive));
    };
    if input.len() < n as usize {
        return Err(Error::NotEnoughBytes);
    }
    let payload = &input[1..n as usize];
    let r = match input[0] {
        0 => Ok(Message(Choke)),
        1 => Ok(Message(Unchoke)),
        2 => Ok(Message(Interested)),
        3 => Ok(Message(NotInterested)),
        4 => Ok(Message(Have(decode::u32(payload).ok_or(Error::Have)?.1))),
        5 => Ok(Message(Bitfield(payload.into()))),
        6 => Ok(Message(Request(
            request::r#try(payload).map_err(Error::Request)?,
        ))),
        7 => Ok(Message(Piece(piece::r#try(payload).map_err(Error::Piece)?))),
        8 => Ok(Message(Cancel(
            request::r#try(payload).map_err(Error::Cancel)?,
        ))),
        _ => Err(Error::NotValidType),
    };
    r.map(|action| (&input[n as usize..], action))
}
