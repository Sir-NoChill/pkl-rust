use rmp_serde as rmps;

use serde::{Serialize, Deserialize};
use rmps::{Serializer, config::{StructMapConfig, self}};

/// Trait required to pass messages to the pkl server.
/// Note this must implement Serialize to work with rmps
pub trait OutgoingMessage: Serialize {}

/// Packs a message in messagepasing v5 format
///
/// # Example
pub fn pack_message(msg: &impl OutgoingMessage, code: u8) -> Result<Vec<u8>, &'static str> {
    let mut buf = Vec::new();
    let value = (code, msg);

    let _ = &value.serialize(&mut Serializer::new(&mut buf).with_struct_map()).unwrap();
    return Ok(buf);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleReader {
    scheme: String,
    has_hierarchical_uris: bool,
    is_globbable: bool,
}

impl OutgoingMessage for ModuleReader {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_message_module_reader() {
        let mr = ModuleReader{scheme: "customfs".into(), has_hierarchical_uris: true, is_globbable: true};
        let msp = pack_message(&mr, 0x20).unwrap();

        let expected = vec![0x92, 0x20, 0x83, 0xA6, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x65, 0xA8, 0x63, 0x75, 0x73, 0x74, 0x6F, 0x6D, 0x66, 0x73, 0xB5, 0x68, 0x61, 0x73, 0x5F, 0x68, 0x69, 0x65, 0x72, 0x61, 0x72, 0x63, 0x68, 0x69, 0x63, 0x61, 0x6C, 0x5F, 0x75, 0x72, 0x69, 0x73, 0xC3, 0xAC, 0x69, 0x73, 0x5F, 0x67, 0x6C, 0x6F, 0x62, 0x62, 0x61, 0x62, 0x6C, 0x65, 0xC3];
        assert_eq!(msp, expected);

        let buf: (u8, ModuleReader) = rmp_serde::from_slice(&msp).unwrap();
        println!("Result of Deserialization: {:?}", buf);
    }
}
