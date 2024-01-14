use crate::decode;

pub type List = Vec<super::Item>;

#[derive(Debug)]
pub enum Error {
    NoEnd,
}

pub fn r#try(input: &[u8]) -> Option<Result<(&[u8], List), super::Error>> {
    let input = decode::char(input, b'l').ok()?;
    Some(end(input))
}

fn end(mut input: &[u8]) -> Result<(&[u8], List), super::Error> {
    let mut items = Vec::new();
    while input.iter().next() != Some(&b'e') {
        if input.iter().next() == None {
            return Err(super::Error::List(Error::NoEnd));
        }
        let (new, value) = super::item(input)?;
        items.push(value);
        input = new;
    }
    Ok((&input[1..], items))
}
