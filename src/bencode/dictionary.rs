use crate::decode;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Dictionary {
    pub items: Vec<(super::Item, super::Item)>,
}

impl serde::Serialize for Dictionary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.items.len()))?;
        for (key, value) in &self.items {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

#[derive(Debug)]
pub enum Error {
    NoEnd,
}

pub fn r#try(input: &[u8]) -> Option<Result<(&[u8], Dictionary), super::Error>> {
    let input = decode::char(input, b'd').ok()?;
    Some(end(input))
}

fn end(mut input: &[u8]) -> Result<(&[u8], Dictionary), super::Error> {
    let mut items = Vec::new();
    while input.iter().next() != Some(&b'e') {
        if input.iter().next() == None {
            return Err(super::Error::Dictionary(Error::NoEnd));
        }
        let (new, key) = super::item(input)?;
        let (new, value) = super::item(new)?;
        items.push((key, value));
        input = new;
    }
    Ok((&input[1..], Dictionary { items }))
}
