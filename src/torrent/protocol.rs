use super::message::*;

pub struct State {
    pub seeding: bool,
    pub interested: bool,
    pub choked: bool,
    pub choking: bool,
    pub bitfield: Vec<u8>,
    pub has: Vec<u8>,
    pub sent_anything: bool,
    pub received_anything: bool,
    pub length: usize,
    pub piece_length: usize,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: usize,
    pub blocks: Option<(usize, Vec<location::Location>)>,
}

const PROTOCOL: &[u8; 20] = b"\x13BitTorrent protocol";

pub fn handshake(info_hash: &[u8; 20], peer_id: &[u8; 20]) -> Vec<u8> {
    vec![
        PROTOCOL.to_vec(),
        [0; 8].to_vec(),
        info_hash.to_vec(),
        peer_id.to_vec(),
    ]
    .concat()
}

pub fn try_handshake<'a>(input: &'a [u8], info_hash: &[u8; 20]) -> Result<(&'a [u8], bool), ()> {
    if input.len() < 68 || input[..20] != PROTOCOL[..] {
        return Err(());
    }
    Ok((&input[68..], input[28..48] == info_hash[..]))
}

fn _find0(bitfield: &Vec<u8>) -> Option<usize> {
    let byte_index = bitfield.iter().position(|x| *x != 255)?;
    let byte = bitfield[byte_index];
    (0..8)
        .position(|i| byte & (1 << i) == 0)
        .map(|i| byte_index * 8 + i)
}

fn find1(bitfield: &Vec<u8>) -> Option<usize> {
    let byte_index = bitfield.iter().position(|x| *x != 0)?;
    let byte = bitfield[byte_index];
    (0..8)
        .position(|i| byte & (1 << i) != 0)
        .map(|i| byte_index * 8 + i)
}

fn count0(length: usize, bitfield: &Vec<u8>) -> usize {
    let mut count = 0;
    for byte in &bitfield[..bitfield.len() - 1] {
        for i in 0..8 {
            if byte & (1 << i) == 0 {
                count += 1;
            }
        }
    }
    let last_byte = bitfield[bitfield.len() - 1];
    for i in 0..(length % 8) {
        if last_byte & (1 << i) == 0 {
            count += 1;
        }
    }
    count
}

fn set(bytes: &mut Vec<u8>, index: usize) {
    bytes[index / 8] |= 1 << (index % 8);
}

fn locations(index: u32, piece_length: u32) -> Vec<location::Location> {
    let mut out = Vec::new();
    let mut begin = 0;
    while begin < piece_length {
        out.push(location::Location { index, begin });
        begin += 16 * 1024;
    }
    out
}

impl State {
    pub fn new(length: usize, piece_length: usize, bitfield: Vec<u8>) -> State {
        let left = count0(length, &bitfield);
        let n = bitfield.len();
        State {
            seeding: false,
            interested: false,
            choked: true,
            choking: true,
            bitfield,
            has: vec![0; n],
            sent_anything: false,
            received_anything: false,
            length,
            piece_length,
            uploaded: 0,
            downloaded: length - left,
            left,
            blocks: None,
        }
    }

    pub fn communicate(&mut self, message: Option<Message>) -> Option<Message> {
        if let Some(message) = message {
            match message {
                Message::Choke => self.choked = true,
                Message::Unchoke => self.choked = false,
                Message::Have(index) => set(&mut self.has, index as usize),
                Message::Bitfield(x) => self.has = x,
                _ => {}
            }
        }
        if !self.sent_anything {
            self.sent_anything = true;
            return Some(Message::Bitfield(self.bitfield.clone()));
        }
        if let Some(index) = find1(&self.has) {
            self.blocks = Some((0, locations(index as u32, self.piece_length as u32)));
        }
        match &mut self.blocks {
            Some((block_index, locations)) => {
                if *block_index >= locations.len() {
                    return None;
                }
                if !self.interested {
                    self.interested = true;
                    return Some(Message::Interested);
                }
                if self.choked {
                    return None;
                }
                /*if self.choking {
                    self.choking = false;
                    return Some(Message::Unchoke);
                }*/
                let location = locations[*block_index].clone();
                *block_index = *block_index + 1;
                Some(Message::Request(request::Request {
                    location,
                    length: 16 * 1024,
                }))
            }
            _ => None,
        }
    }
}
