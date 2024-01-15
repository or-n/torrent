#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyInput,
    NoDelimeter(u8),
    Char(Mismatch<u8>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Mismatch<T> {
    expect: T,
    got: T,
}

pub fn char(input: &[u8], expect: u8) -> Result<&[u8], Error> {
    let got = *input.iter().next().ok_or(Error::EmptyInput)?;
    if got == expect {
        Ok(&input[1..])
    } else {
        Err(Error::Char(Mismatch { expect, got }))
    }
}

pub fn till(input: &[u8], delimeter: u8) -> Result<(&[u8], &[u8]), Error> {
    match input.iter().position(|&x| x == delimeter) {
        Some(i) => Ok((&input[i + 1..], &input[..i])),
        _ => Err(Error::NoDelimeter(delimeter)),
    }
}

pub fn wrap<Input, Ok, NewOk, Err, NewErr>(
    ok: impl FnOnce(Ok) -> NewOk,
    err: impl FnOnce(Err) -> NewErr,
) -> impl FnOnce(Result<(Input, Ok), Err>) -> Result<(Input, NewOk), NewErr> {
    move |result| match result {
        Ok((input, x)) => Ok((input, ok(x))),
        Err(e) => Err(err(e)),
    }
}

pub fn u32(input: &[u8]) -> Option<(&[u8], u32)> {
    if input.len() < 4 {
        None
    } else {
        Some((
            &input[4..],
            u32::from_be_bytes([input[0], input[1], input[2], input[3]]),
        ))
    }
}

pub fn many<Ok, Error>(
    decoder: impl Fn(&[u8]) -> Result<(&[u8], Ok), Error>,
) -> impl Fn(&[u8]) -> (&[u8], Vec<Ok>, Error) {
    move |mut input| {
        let mut out = Vec::new();
        loop {
            match decoder(input) {
                Ok((new_input, x)) => {
                    out.push(x);
                    input = new_input;
                }
                Err(e) => return (input, out, e),
            }
        }
    }
}
