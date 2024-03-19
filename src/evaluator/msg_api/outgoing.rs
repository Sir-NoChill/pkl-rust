#![allow(nonstandard_style)] // for compatibililty with pkl messages
use std::collections::HashMap;

use rmp_serde as rmps;

use serde::Serialize;
use rmps::Serializer;

/// Packs a message in messagepasing v5 format
///
/// # Example
pub fn pack_message(msg: &impl Serialize, code: u8) -> Result<Vec<u8>, &'static str> {
    let mut buf = Vec::new();
    let value = (code, msg);

    let _ = &value.serialize(&mut Serializer::new(&mut buf).with_struct_map()).unwrap();
    return Ok(buf);
}

#[derive(Debug, Serialize)]
pub struct ModuleReader {
    scheme: String,
    hasHierarchicalUris: bool,
    isGlobbable: bool,
    isLocal: bool,
}

#[derive(Debug, Serialize)]
pub struct ResourceReader {
    scheme: String,
    hasHierarchicalUris: bool,
    isGlobbable: bool,
}

#[derive(Debug, Serialize)]
pub struct Checksums {
    checksums: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectOrDependency {
    packageUri: Option<String>,
    r#type: String,
    projectFileUri: Option<String>,
    checksums: Option<Checksums>,
    dependencies: HashMap<String, ProjectOrDependency>,
}

#[derive(Debug, Serialize)]
pub struct CreateEvaluator {
    requestId: i64,
    clientResourceReaders: Option<Vec<ResourceReader>>,
    clientModuleReaders: Option<Vec<ModuleReader>>,
    modulePaths: Option<Vec<String>>,
    env: Option<HashMap<String, String>>,
    properties: Option<HashMap<String, String>>,
    outputFormat: Option<String>,
    allowedModules: Option<Vec<String>>,
    allowedResources: Option<Vec<String>>,
    rootDir: Option<String>,
    cacheDir: Option<String>,
    project: Option<ProjectOrDependency>,
    timeoutSeconds: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CloseEvaluator {
    evaluatorId: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct Evaluate {
    requestId: i64,
    evaluatorId: i64,
    moduleUri: String,
    moduleText: Option<String>,
    expr: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReadResourceResponse {
    requestId: i64,
    evaluatorId: i64,
    contents: Option<Vec<u8>>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReadModuleResponse {
    requestId: i64,
    evaluatorId: i64,
    contents: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListResourceResponse {
    requestId: i64,
    evaluatorId: i64,
    pathElements: Option<Vec<PathElement>>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListModulesResponse {
    requestId: i64,
    evaluatorId: i64,
    pathElements: Option<Vec<PathElement>>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PathElement {
    name: String,
    isDirectory: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluator::msg_api::code;

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

    #[test]
    /// Test against the specification docs
    /// [
    ///  0x2d,
    ///  {
    ///    "requestId": -6478924,
    ///    "evaluatorId": -13901,
    ///    "pathElements": [
    ///      {
    ///        "name": "foo.pkl",
    ///        "isDirectory": false
    ///      }
    ///     "error": null  // note that this is the same as ommitting the element
    ///    ]
    ///   }
    /// ]
    fn test_pack_message_specification_1() {
        let pe = PathElement{name: "foo.pkl".into(), isDirectory: false};
        let mr = ListModulesResponse{requestId: -647892, evaluatorId: -13901, pathElements: vec![pe].into(), error: None};

        let mp = pack_message(&mr, code::CODE_LIST_MODULES_RESPONSE).unwrap();
        let expected = vec![0x92, 0x2D, 0x84, 0xA9, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x64,
                            0xD2, 0xFF, 0xF6, 0x1D, 0x2C, 0xAB, 0x65, 0x76, 0x61, 0x6C, 0x75, 0x61, 0x74,
                            0x6F, 0x72, 0x49, 0x64, 0xD1, 0xC9, 0xB3, 0xAC, 0x70, 0x61, 0x74, 0x68, 0x45,
                            0x6C, 0x65, 0x6D, 0x65, 0x6E, 0x74, 0x73, 0x91, 0x82, 0xA4, 0x6E, 0x61, 0x6D,
                            0x65, 0xA7, 0x66, 0x6F, 0x6F, 0x2E, 0x70, 0x6B, 0x6C, 0xAB, 0x69, 0x73, 0x44,
                            0x69, 0x72, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x79, 0xC2, 0xA5, 0x65, 0x72, 0x72, 0x6F, 0x72, 0xC0];

        println!("Serialized: {:X?}", mp);

        assert_eq!(mp, expected);
    }
}
