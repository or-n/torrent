use crate::bencode;
use crate::util;

pub mod announce_list;
pub mod info;
pub mod url_list;

pub struct Meta {
    pub announce: Option<String>,
    pub announce_list: Option<announce_list::AnnounceList>,
    pub comment: (),
    pub created_by: (),
    pub creation_date: (),
    pub info: bencode::Item,
    pub url_list: Option<Vec<String>>,
}

use util::bencode::{field, FieldError};

#[derive(Debug)]
pub enum Error {
    NotDictionary,
    Announce(FieldError),
    AnnounceList(announce_list::Error),
    Info(FieldError),
    UrlList(url_list::Error),
}

pub fn extract(item: &bencode::Item) -> Result<Meta, Error> {
    let d = util::bencode::dictionary(item).ok_or(Error::NotDictionary)?;
    Ok(Meta {
        announce: field("announce", d)
            .ok()
            .map(|item| util::bencode::string(item).ok_or(FieldError::NotValid))
            .transpose()
            .map_err(Error::Announce)?
            .map(|bytes| format!("{:?}", bytes)),
        announce_list: field("announce-list", d)
            .ok()
            .map(announce_list::r#try)
            .transpose()
            .map_err(Error::AnnounceList)?,
        comment: (),
        created_by: (),
        creation_date: (),
        info: field("info", d).map_err(Error::Info)?.clone(),
        url_list: field("url-list", d)
            .ok()
            .map(url_list::r#try)
            .transpose()
            .map_err(Error::UrlList)?,
    })
}
