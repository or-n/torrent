use reqwest;
use sha1::{Digest, Sha1};

pub mod bencode;

pub fn lookup<'a, Key: Ord, Value: Clone>(
    xs: &'a Vec<(Key, Value)>,
    key: Key,
) -> Option<&'a Value> {
    xs.binary_search_by(|x| x.0.cmp(&key))
        .ok()
        .map(|index| &(&xs[index]).1)
}

pub fn sha1(bytes: Vec<u8>) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

pub fn percent_hex_string(array: &[u8]) -> String {
    array.iter().map(|&byte| format!("%{:02x}", byte)).collect()
}

pub async fn fetch_bytes(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let bytes = reqwest::get(url).await?.bytes().await?;
    Ok(bytes.to_vec())
}
