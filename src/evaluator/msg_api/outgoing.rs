use rmp;

/// Trait required to pass messages to the pkl server.
trait OutgoingMessage {
    fn to_msg_pack(&self) -> Result<&[u8], &'static str>;
}

/// Packs a message in messagepasing v5 format
///
/// # Example
pub fn pack_message(msg: &impl OutgoingMessage, code: u8) -> Result<Vec<u8>, &'static str> {
    let mut buf = Vec::new();
    match rmp::encode::write_u8(&mut buf, code) {
        Err(_) => return Err("Failed to encode message passing code in message pack"),
        _ => {},
    };
    match rmp::encode::write_bin(&mut buf, msg.to_msg_pack()
                .expect("Failed to convert message passing msg to message pack binary representation")) {
        Err(_) => return Err("Failed to encode mess"),
        _ => {},
    }

    return Ok(buf);
}
