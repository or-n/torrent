use crate::bencode;

fn _top_string(x: &bencode::Item) -> String {
    (match x {
        bencode::Item::Dictionary(_) => "Dictionary",
        bencode::Item::List(_) => "List",
        bencode::Item::Integer(_) => "Integer",
        bencode::Item::String(_) => "String",
    })
    .to_string()
}

pub fn _print_top(item: &bencode::Item) {
    match item {
        bencode::Item::Dictionary(d) => {
            for (key, value) in &d.items {
                if let bencode::Item::String(bytes) = key {
                    println!("{:?}: {}", bytes, _top_string(&value));
                }
            }
        }
        bencode::Item::List(xs) => {
            for value in xs {
                println!("{}", _top_string(&value));
            }
        }
        _ => {}
    }
}

pub fn key(name: &str) -> bencode::Item {
    bencode::Item::String(bencode::string::Bytes {
        bytes: name.as_bytes().into(),
    })
}

macro_rules! extract {
    ($item:expr, $variant:pat => $result:expr) => {
        if let $variant = $item {
            Some($result)
        } else {
            None
        }
    };
}

pub fn string(item: &bencode::Item) -> Option<&bencode::string::Bytes> {
    extract!(item, bencode::Item::String(bytes) => bytes)
}

pub fn integer(item: &bencode::Item) -> Option<i64> {
    extract!(item, bencode::Item::Integer(x) => *x)
}

pub fn list(item: &bencode::Item) -> Option<&bencode::list::List> {
    extract!(item, bencode::Item::List(l) => l)
}

pub fn dictionary(item: &bencode::Item) -> Option<&bencode::dictionary::Dictionary> {
    extract!(item, bencode::Item::Dictionary(d) => d)
}

pub fn json(item: &bencode::Item) -> String {
    let object = serde_json::json!(item);
    serde_json::to_string_pretty(&object).expect("pretty json")
}

#[derive(Debug)]
pub enum FieldError {
    NotFound,
    NotValid,
}

type Dictionary = bencode::dictionary::Dictionary;

pub fn field<'a>(key_name: &str, d: &'a Dictionary) -> Result<&'a bencode::Item, FieldError> {
    super::lookup(&d.items, key(key_name)).ok_or(FieldError::NotFound)
}

pub fn integer_field(key_name: &str, d: &Dictionary) -> Result<i64, FieldError> {
    integer(field(key_name, d)?).ok_or(FieldError::NotValid)
}

pub fn string_field(key_name: &str, d: &Dictionary) -> Result<bencode::string::Bytes, FieldError> {
    string(field(key_name, d)?)
        .ok_or(FieldError::NotValid)
        .cloned()
}
