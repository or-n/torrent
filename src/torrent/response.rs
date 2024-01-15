use crate::bencode;
use crate::util;

pub struct Response {
    pub complete: i64,
    //pub downloaded: i64,
    pub incomplete: i64,
    //pub min_interval: i64,
    pub peers: Vec<Peer>,
}

use util::bencode::{integer_field, string_field, FieldError};

#[derive(Debug)]
pub enum Error {
    NotDictionary,
    Complete(FieldError),
    //Downloaded(FieldError),
    Incomplete(FieldError),
    //MinInterval(FieldError),
    Peers(FieldError),
}

pub fn extract(item: &bencode::Item) -> Result<Response, Error> {
    let d = util::bencode::dictionary(item).ok_or(Error::NotDictionary)?;
    Ok(Response {
        complete: integer_field("complete", d).map_err(Error::Complete)?,
        //downloaded: integer_field("downloaded", d).map_err(Error::Downloaded)?,
        incomplete: integer_field("incomplete", d).map_err(Error::Incomplete)?,
        //min_interval: integer_field("min interval", d).map_err(Error::MinInterval)?,
        peers: string_field("peers", d)
            .map_err(Error::Peers)?
            .bytes
            .chunks(6)
            .map(|x| x.try_into().map(Peer))
            .flatten()
            .collect(),
    })
}

#[derive(Clone)]
pub struct Peer(pub [u8; 6]);

impl std::fmt::Debug for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.0;
        let ip = std::net::Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]);
        let port = u16::from_be_bytes([bytes[4], bytes[5]]);
        write!(f, "{}:{}", ip, port)
    }
}
