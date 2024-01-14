use crate::bencode;
use crate::util;

pub mod info;

pub struct Meta {
    pub announce: String,
    pub announce_list: (),
    pub comment: (),
    pub created_by: (),
    pub creation_date: (),
    pub info: bencode::Item,
    pub url_list: (),
}

use util::bencode::{field, string_field, FieldError};

#[derive(Debug)]
pub enum Error {
    NotDictionary,
    Announce(FieldError),
    Info(FieldError),
}

pub fn extract(item: &bencode::Item) -> Result<Meta, Error> {
    let d = util::bencode::dictionary(item).ok_or(Error::NotDictionary)?;
    Ok(Meta {
        announce: format!(
            "{:?}",
            string_field("announce", d).map_err(Error::Announce)?
        ),
        announce_list: (),
        comment: (),
        created_by: (),
        creation_date: (),
        info: field("info", d).map_err(Error::Info)?.clone(),
        url_list: (),
    })
}
