# BitTorrent client in Rust

## Example usage

```console
RUST_LOG=info cargo run --release -- -t example.torrent --verbose
```

## Description

It reads the specified .torrent files to find the HTTP trackers.
Then it requests peers from the first one that responds.
Then it tries each peer until it find one that connects over TCP.
Then it exchanges handshakes and sends empty bitfield.
It waits till it is unchoked and knows that the peer has some pieces.
Then it tries to request and receive all blocks of some piece.
It doesn't find another peer when one disconnects.
