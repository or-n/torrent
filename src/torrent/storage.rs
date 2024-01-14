pub fn new_state(info: &super::meta::info::Info) -> super::protocol::State {
    let ceil_div = |a, b| (a + b - 1) / b;
    let n_bytes = ceil_div(ceil_div(info.length, info.piece_length), 8);
    let bitfield = vec![0; n_bytes as usize];
    super::protocol::State::new(bitfield)
}
