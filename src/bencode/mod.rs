use crate::decode;

pub mod dictionary;
pub mod integer;
pub mod list;
pub mod string;

#[derive(Debug)]
pub enum Error {
    String(string::Error),
    Integer(integer::Error),
    List(list::Error),
    Dictionary(dictionary::Error),
    NotAnItem,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, serde::Serialize)]
#[serde(untagged)]
pub enum Item {
    String(string::Bytes),
    Integer(i64),
    List(list::List),
    Dictionary(dictionary::Dictionary),
}

pub fn item(input: &[u8]) -> Result<(&[u8], Item), Error> {
    if let Some(result) = list::r#try(input) {
        return decode::wrap(Item::List, |error| error)(result);
    }
    if let Some(result) = dictionary::r#try(input) {
        return decode::wrap(Item::Dictionary, |error| error)(result);
    }
    if let Some(result) = integer::r#try(input) {
        return decode::wrap(Item::Integer, Error::Integer)(result);
    }
    if let Some(result) = string::r#try(input) {
        return decode::wrap(Item::String, Error::String)(result);
    }
    Err(Error::NotAnItem)
}

pub fn encode(item: &Item) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_to(&mut bytes, item);
    bytes
}

fn encode_to(output: &mut Vec<u8>, item: &Item) {
    match item {
        Item::String(x) => {
            let bytes = &x.bytes;
            output.extend(format!("{}:", bytes.len()).as_bytes());
            output.extend(bytes);
        }
        Item::Integer(x) => output.extend(format!("i{}e", x).as_bytes()),
        Item::List(xs) => {
            output.push(b'l');
            for value in xs {
                encode_to(output, value);
            }
            output.push(b'e');
        }
        Item::Dictionary(d) => {
            output.push(b'd');
            for (key, value) in &d.items {
                encode_to(output, key);
                encode_to(output, value);
            }
            output.push(b'e');
        }
    }
}
