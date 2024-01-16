use crate::bencode;
use crate::util;

#[derive(Debug)]
pub enum Error {
    NotList,
    ItemNotString,
}

pub fn r#try(item: &bencode::Item) -> Result<Vec<String>, Error> {
    Ok(util::bencode::list(item)
        .ok_or(Error::NotList)?
        .into_iter()
        .map(|item| util::bencode::string(item).map(|x| format!("{:?}", x)))
        .collect::<Option<_>>()
        .ok_or(Error::ItemNotString)?)
}
