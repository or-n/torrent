use rustorrent::bencode;

const EMPTY: &[u8] = &[];

#[test]
fn test_string_bencode() {
    assert_eq!(
        bencode::item(b"7:torrent"),
        Ok((
            EMPTY,
            bencode::Item::String(bencode::string::Bytes {
                bytes: b"torrent".to_vec()
            })
        ))
    );
}

#[test]
fn test_integer() {
    assert_eq!(
        bencode::item(b"i0e"),
        Ok((EMPTY, bencode::Item::Integer(0)))
    );
    assert_eq!(
        bencode::integer::r#try(b"i01e"),
        Some(Err(bencode::integer::Error::LeadingZeroes))
    );
    assert_eq!(
        bencode::integer::r#try(b"i-0e"),
        Some(Err(bencode::integer::Error::NegativeZero))
    );
    assert_eq!(
        bencode::item(b"i-1e"),
        Ok((EMPTY, bencode::Item::Integer(-1)))
    );
    match bencode::item(b"ie") {
        Err(bencode::Error::Integer(bencode::integer::Error::ParseInt(e))) => {
            assert_eq!(e.to_string(), "cannot parse integer from empty string");
        }
        other => panic!("Unexpected result: {:?}", other),
    }
}

#[test]
fn test_list() {
    assert_eq!(
        bencode::item(b"li42e7:torrente"),
        Ok((
            EMPTY,
            bencode::Item::List(vec![
                bencode::Item::Integer(42),
                bencode::Item::String(bencode::string::Bytes {
                    bytes: b"torrent".to_vec()
                })
            ])
        ))
    );
}

#[test]
fn test_dictionary() {
    assert_eq!(
        bencode::item(b"d7:torrent10:my-torrent4:sizei150ee"),
        Ok((
            EMPTY,
            bencode::Item::Dictionary(bencode::dictionary::Dictionary {
                items: vec![
                    (
                        bencode::Item::String(bencode::string::Bytes {
                            bytes: b"torrent".to_vec()
                        }),
                        bencode::Item::String(bencode::string::Bytes {
                            bytes: b"my-torrent".to_vec()
                        })
                    ),
                    (
                        bencode::Item::String(bencode::string::Bytes {
                            bytes: b"size".to_vec()
                        }),
                        bencode::Item::Integer(150)
                    )
                ]
            })
        ))
    );
}
