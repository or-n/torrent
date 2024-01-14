use super::message::*;

pub struct State {
    pub seeding: bool,
    pub interested: bool,
    pub choked: bool,
    pub bitfield: Vec<u8>,
    pub sent_anything: bool,
    pub received_anything: bool,
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

fn find0(bitfield: &Vec<u8>) -> Option<usize> {
    let byte_index = bitfield.iter().position(|x| x != &255)?;
    let byte = bitfield[byte_index];
    (0..8)
        .position(|i| byte & (1 << i) == 0)
        .map(|i| byte_index * 8 + i)
}

impl State {
    pub fn new(bitfield: Vec<u8>) -> State {
        State {
            seeding: false,
            interested: false,
            choked: false,
            bitfield,
            sent_anything: false,
            received_anything: false,
        }
    }

    pub fn communicate(&mut self, message: Option<Message>) -> Option<Message> {
        if let Some(message) = message {
            match message {
                Message::Choke => self.choked = true,
                _ => {}
            }
        }
        if !self.sent_anything {
            self.sent_anything = true;
            return Some(Message::Bitfield(self.bitfield.clone()));
        }
        match find0(&self.bitfield) {
            Some(index) => {
                if !self.interested {
                    self.interested = true;
                    return Some(Message::Interested);
                }
                if self.choked {
                    None
                } else {
                    let location = location::Location { index: 0, begin: 0 };
                    Some(Message::Request(request::Request {
                        location,
                        length: 16 * 1024,
                    }))
                }
            }
            _ => None,
        }
    }
}