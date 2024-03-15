#![allow(nonstandard_style)]
use std::collections::HashMap;

use rmp_serde as rmps;

use serde::{Serialize, Deserialize};
use rmps::Serializer;

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
    hasHierarchicalUris: bool,
    isGlobbable: bool,
    isLocal: bool,
}
impl OutgoingMessage for ModuleReader {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceReader {
    scheme: String,
    hasHierarchicalUris: bool,
    isGlobbable: bool,
}
impl OutgoingMessage for ResourceReader {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Checksums {
    checksums: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectOrDependency {
    packageUri: String,
    r#type: String,
    projectFileUri: String,
    checksums: Checksums,
    dependencies: HashMap<String, ProjectOrDependency>,
}
impl OutgoingMessage for ProjectOrDependency {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEvaluator {
    requestId: i64,
    clientResourceReaders: Vec<ResourceReader>,
    clientModuleReaders: Vec<ModuleReader>,
    modulePaths: Vec<String>,
    env: HashMap<String, String>,
    properties: HashMap<String, String>,
    outputFormat: String,
    allowedModules: Vec<String>,
    allowedResources: Vec<String>,
    rootDir: String,
    cacheDir: String,
    project: ProjectOrDependency,
}
impl OutgoingMessage for CreateEvaluator {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Evaluate {
    requestId: i64,
    evaluatorId: i64,
    moduleUri: String,
    moduleText: String,
    expr: String,
}
impl OutgoingMessage for Evaluate {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadResourceResponse {
    requestId: i64,
    evaluatorId: i64,
    contents: Vec<u8>,
    error: String,
}
impl OutgoingMessage for ReadResourceResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadModuleResponse {
    requestId: i64,
    evaluatorId: i64,
    contents: String,
    error: String,
}
impl OutgoingMessage for ReadModuleResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathElement {
    name: String,
    isDirectory: bool,
}
impl OutgoingMessage for PathElement {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResourceResponse {
    requestId: i64,
    evaluatorId: i64,
    pathElements: Vec<PathElement>,
    error: String,
}
impl OutgoingMessage for ListResourceResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModulesResponse {
    requestId: i64,
    evaluatorId: i64,
    pathElements: Vec<PathElement>,
    error: String,
}
impl OutgoingMessage for ListModulesResponse {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_message_module_reader() {
        let mr = ModuleReader{scheme: "customfs".into(), hasHierarchicalUris: true, isGlobbable: true, isLocal: false };
        let msp = pack_message(&mr, 0x20).unwrap();

        let expected = vec![0x92, 0x20, 0x84, 0xA6, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x65, 0xA8, 0x63, 0x75, 0x73, 0x74,
                            0x6F, 0x6D, 0x66, 0x73, 0xB3, 0x68, 0x61, 0x73, 0x48, 0x69, 0x65, 0x72, 0x61, 0x72, 0x63,
                            0x68, 0x69, 0x63, 0x61, 0x6C, 0x55, 0x72, 0x69, 0x73, 0xC3, 0xAB, 0x69, 0x73, 0x47, 0x6C,
                            0x6F, 0x62, 0x62, 0x61, 0x62, 0x6C, 0x65, 0xC3, 0xA7, 0x69, 0x73, 0x4C, 0x6F, 0x63, 0x61, 0x6C, 0xC2];

        assert_eq!(msp, expected);
    }
}
