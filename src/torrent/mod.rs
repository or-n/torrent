pub mod message;
pub mod meta;
pub mod protocol;
pub mod response;
pub mod storage;

use crate::bencode;
use crate::util;

use std::fmt::Display;

fn param<Value: Display>(key: &str, value: Value) -> String {
    format!("{}={}", key, value)
}

pub fn query(peer_id: &[u8; 20], meta: &meta::Meta) -> (String, [u8; 20]) {
    let info_hash = util::sha1(bencode::encode(&meta.info));
    let params = [
        param("info_hash", util::percent_hex_string(&info_hash)),
        param("peer_id", String::from_utf8_lossy(peer_id).to_string()),
        param("port", 6881),
        param("uploaded", 0),
        param("downloaded", 0),
        param("left", 0),
        param("compact", 1),
    ];
    (format!("{}?{}", meta.announce, params.join("&")), info_hash)
}
