/*println!("{:?}", bencode::item(b"7:torrent"));
for x in ["0", "01", "-0", "-1", "-01", ""].into_iter() {
    let input = format!("i{}e", x);
    println!("{}: {:?}", input, bencode::item(input.as_bytes()));
}
println!("{:?}", bencode::item(b"li42e7:torrente"));
println!(
    "{:?}",
    bencode::item(b"d7:torrent10:my-torrent4:sizei150ee")
);

let mut bytes2 = Vec::new();
bencode::encode(meta, &mut bytes2);
println!("are bytes same: {}", bytes == bytes2);
*/
