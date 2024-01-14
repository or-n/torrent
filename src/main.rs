mod bencode;
mod decode;
mod torrent;
mod util;

use clap::Parser;
use log::info;

/// BitTorrent client
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Path to .torrent
    #[arg(short, long)]
    torrent: Vec<String>,

    /// Print metainfo of torrents instead of torrenting
    #[arg(short, long)]
    pretty_print_torrent_file: bool,

    /// Print peers returned by the tracker
    #[arg(short, long)]
    dump_peers: bool,

    /// Log what is happening
    #[arg(short, long)]
    verbose: bool,
}

const PEER_ID: &[u8; 20] = b"-MB-2025-aaaaaaaaaaa";

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.verbose {
        env_logger::init();
    }
    let pretty_print = args.pretty_print_torrent_file;
    for torrent in args.torrent {
        let bytes = std::fs::read(&torrent).expect(format!("file: {}", torrent).as_str());
        let (_, meta_item) = bencode::item(&bytes).expect("bencode meta");
        if pretty_print {
            println!("{}", util::bencode::json(&meta_item));
        } else {
            let meta = torrent::meta::extract(&meta_item).expect("valid meta");
            let info = torrent::meta::info::extract(&meta.info).expect("valid info");
            let (url, info_hash) = torrent::query(PEER_ID, &meta);
            info!("{torrent}: tracker: requesting peers to {}", meta.announce);
            let bytes = util::fetch_bytes(url).await.expect("http protocol");
            let (_, response_item) = bencode::item(&bytes).expect("bencode response");
            let response = torrent::response::extract(&response_item).expect("valid response");
            if args.dump_peers {
                for peer in &response.peers {
                    println!("{:?}", peer);
                }
            }
            torrenting(torrent, &info_hash, &info, &response).await;
        }
    }
}

use std::time::Duration;
use tokio::io::{AsyncWriteExt, ErrorKind};
use tokio::net::TcpStream;

async fn connect(
    peers: &Vec<torrent::response::Peer>,
) -> Option<(torrent::response::Peer, TcpStream)> {
    for peer in peers {
        tokio::select! {
            connected = TcpStream::connect(format!("{:?}", peer)) => {
                if let Ok(stream) = connected {
                    return Some((peer.clone(), stream));
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(1)) => {}
        }
    }
    None
}

async fn torrenting(
    torrent: String,
    info_hash: &[u8; 20],
    info: &torrent::meta::info::Info,
    response: &torrent::response::Response,
) {
    let (peer, mut stream) = connect(&response.peers).await.expect("TCP");
    info!("{torrent}: peers: connect {:?}", peer);
    let handshake = torrent::protocol::handshake(&info_hash, PEER_ID);
    stream.write_all(&handshake).await.expect("handshake");
    let recv = format!("{torrent}: msg: recv {:?}", peer);
    let send = format!("{torrent}: msg: send {:?}", peer);
    info!("{} handshake", send);
    let state = torrent::storage::new_state(info);
    handle_peer((recv, send), stream, state, &info_hash).await;
    info!("{torrent}: peers: disconnect {:?}", peer);
}

async fn handle_peer(
    (recv, send): (String, String),
    mut stream: TcpStream,
    mut state: torrent::protocol::State,
    info_hash: &[u8; 20],
) {
    let mut buffer = vec![0; 32 * 1024];
    loop {
        tokio::select! {
            readable = stream.readable() => {
                readable.expect("readable");
                match stream.try_read(&mut buffer) {
                    Ok(0) => {
                        return;
                    }
                    Ok(n) => {
                        let mut bytes = &buffer[..n];
                        if let Ok((new_bytes, valid_hash)) = torrent::protocol::try_handshake(bytes, info_hash) {
                            if valid_hash {
                                info!("{recv} handshake");
                                bytes = new_bytes;
                            } else {
                                return;
                            }
                        }
                        let (_, actions) = decode::many(torrent::message::r#try)(bytes);
                        for action in actions {
                            use torrent::message::Action;
                            match action {
                                Action::KeepAlive => {}
                                Action::Message(message) => {
                                    info!("{recv} {:?}", message);
                                    if let Some(m) = state.communicate(Some(message)) {
                                        stream.write_all(&m.encode()).await.expect("send");
                                        info!("{send} {:?}", m);
                                    }
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(2)) => {
                //info!("didn't receive anything in 2s");
                if let Some(m) = state.communicate(None) {
                    stream.write_all(&m.encode()).await.expect("send");
                    info!("{send} {:?}", m);
                }
            }
        };
    }
}
