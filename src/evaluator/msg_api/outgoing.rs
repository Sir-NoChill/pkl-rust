use rmp_serde as rmps;

use serde::{Serialize, Deserialize};
use rmps::Serializer;

/// Trait required to pass messages to the pkl server.
/// Note this must implement Serialize to work with rmps
pub trait OutgoingMessage: Serialize {
    fn to_msg_pack(&self) -> Result<Vec<u8>, &'static str>;
}

/// Packs a message in messagepasing v5 format
///
/// # Example
pub fn pack_message(msg: &impl OutgoingMessage, code: u8) -> Result<Vec<u8>, &'static str> {
    let mut buf = Vec::new();
    let value = (code, msg);

    let _ = &value.serialize(&mut Serializer::new(&mut buf)).unwrap();
    return Ok(buf);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleReader {
    scheme: String,
    has_hierarchical_uris: bool,
    is_globbable: bool,
}

impl OutgoingMessage for ModuleReader {
    fn to_msg_pack(&self) -> Result<Vec<u8>, &'static str> {
        let mut buf = Vec::new();

        match &self.serialize(&mut Serializer::new(&mut buf).with_human_readable()) {
            Ok(_) => return Ok(buf),
            Err(_) => return Err("Failed to serialize"),
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_message_module_reader() {
        let mr = ModuleReader{scheme: "customfs".into(), has_hierarchical_uris: true, is_globbable: true};
        let msp = pack_message(&mr, 0x20).unwrap();

        let expected = vec![0x92, 0x1e, 0x83, 0xa6, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0xa8, 0x63, 0x75, 0x73, 0x74, 0x6f,
                            0x6d, 0x66, 0x73, 0xa9, 0x67, 0x6c, 0x6f, 0x62, 0x62, 0x61, 0x62, 0x6c, 0x65, 0xc3, 0xad, 0x68,
                            0x61, 0x73, 0x4e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x55, 0x72, 0x69, 0x6c, 0xc3,];

        println!("Result of Serialization: {:X?}", msp);

        let buf: (u8, ModuleReader) = rmp_serde::from_slice(&msp).unwrap();
        println!("Result of Deserialization: {:?}", buf);
    }
}
