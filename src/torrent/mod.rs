pub mod message;
pub mod meta;
pub mod protocol;
pub mod response;
pub mod storage;

use crate::util;

use std::fmt::Display;

fn param<Value: Display>(key: &str, value: Value) -> String {
    format!("{}={}", key, value)
}

pub fn query(
    peer_id: &[u8; 20],
    info_hash: &[u8; 20],
    announce: &String,
    state: &protocol::State,
) -> String {
    let params = [
        param("info_hash", util::percent_hex_string(&info_hash[..])),
        param("peer_id", String::from_utf8_lossy(peer_id).to_string()),
        param("port", 6881),
        param("uploaded", state.uploaded),
        param("downloaded", state.downloaded),
        param("left", state.left),
        param("compact", 1),
    ];
    format!("{}?{}", announce, params.join("&"))
}
