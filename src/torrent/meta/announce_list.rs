use crate::bencode;
use crate::util;

use super::url_list;

pub type AnnounceList = Vec<Vec<String>>;

#[derive(Debug)]
pub enum Error {
    NotList,
    Item(super::url_list::Error),
}

pub fn r#try(item: &bencode::Item) -> Result<AnnounceList, Error> {
    Ok(util::bencode::list(item)
        .ok_or(Error::NotList)?
        .into_iter()
        .map(url_list::r#try)
        .collect::<Result<_, _>>()
        .map_err(Error::Item)?)
}
