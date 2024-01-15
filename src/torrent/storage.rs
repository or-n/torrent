fn ceil_div(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

pub fn new_state(info: &super::meta::info::Info) -> super::protocol::State {
    let bitfield =
        load_bitfield(info.name.as_str(), info.length, info.piece_length).expect("bitfield");
    super::protocol::State::new(info.length as usize, info.piece_length as usize, bitfield)
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    PiecesMismatch(usize, usize),
}

pub fn load_bitfield(path: &str, length: i64, piece_length: i64) -> Result<Vec<u8>, Error> {
    let pieces = ceil_div(ceil_div(length, piece_length), 8) as usize;
    let progress = &format!("{}.progress", path);
    match std::fs::metadata(progress) {
        Ok(_) => {
            let bitfield = std::fs::read(progress).map_err(Error::IO)?;
            if bitfield.len() != pieces {
                return Err(Error::PiecesMismatch(bitfield.len(), pieces));
            }
            Ok(bitfield)
        }
        Err(_) => {
            let bitfield = vec![0; pieces];
            std::fs::write(progress, &bitfield).map_err(Error::IO)?;
            Ok(bitfield)
        }
    }
}
