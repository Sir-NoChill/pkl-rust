#![allow(nonstandard_style)] // for compatibililty with pkl messages
use std::collections::HashMap;

use rmp_serde as rmps;

use serde::Serialize;
use rmps::Serializer;

use super::code::{CODE_NEW_EVALUATOR, CODE_NEW_EVALUATOR_RESPONSE, CODE_CLOSE_EVALUATOR, CODE_EVALUATE, CODE_EVALUATE_RESPONSE, CODE_EVALUATE_READ_RESPONSE, CODE_EVALUATE_READ_MODULE_RESPONSE, CODE_LIST_RESOURCES_RESPONSE, CODE_LIST_MODULES_REQUEST, CODE_LIST_MODULES_RESPONSE};

/// Packs a message in messagepasing v5 format
///
/// # Example
pub fn pack_message(msg: OutgoingMessage) -> Result<Vec<u8>, &'static str> {
    let mut buf = Vec::new();
    let code = get_code(&msg).0;
    let value = (code, &msg);

    let _ = &value.serialize(&mut Serializer::new(&mut buf).with_struct_map().with_binary()).unwrap();
    return Ok(buf);
}

fn get_code(t: &OutgoingMessage) -> (u8, Option<u8>) {
    match t {
        OutgoingMessage::CreateEvaluator(..) => (CODE_NEW_EVALUATOR, Some(CODE_NEW_EVALUATOR_RESPONSE)),
        OutgoingMessage::CloseEvaluator(..) => (CODE_CLOSE_EVALUATOR, None),
        OutgoingMessage::Evaluate(..) => (CODE_EVALUATE, Some(CODE_EVALUATE_RESPONSE)),
        OutgoingMessage::ReadResourceResponse(..) => (CODE_EVALUATE_READ_RESPONSE, None),
        OutgoingMessage::ReadModuleResponse(..) => (CODE_EVALUATE_READ_MODULE_RESPONSE, None),
        OutgoingMessage::ListResourceResponse(..) => (CODE_LIST_RESOURCES_RESPONSE, None),
        OutgoingMessage::ListModulesResponse(..) => (CODE_LIST_MODULES_RESPONSE, None),
    }
}

pub enum OutgoingMessage {
    CreateEvaluator(CreateEvaluator),
    CloseEvaluator(CloseEvaluator),
    Evaluate(Evaluate),
    ReadResourceResponse(ReadResourceResponse),
    ReadModuleResponse(ReadModuleResponse),
    ListResourceResponse(ListResourceResponse),
    ListModulesResponse(ListModulesResponse),
}

impl Serialize for OutgoingMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            OutgoingMessage::CreateEvaluator(v) => v.serialize(serializer),
            OutgoingMessage::CloseEvaluator(v) => v.serialize(serializer),
            OutgoingMessage::Evaluate(v) => v.serialize(serializer),
            OutgoingMessage::ReadResourceResponse(v) => v.serialize(serializer),
            OutgoingMessage::ReadModuleResponse(v) => v.serialize(serializer),
            OutgoingMessage::ListResourceResponse(v) => v.serialize(serializer),
            OutgoingMessage::ListModulesResponse(v) => v.serialize(serializer),
        }
    }
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
    pub scheme: String,
    pub hasHierarchicalUris: bool,
    pub isGlobbable: bool,
}

#[derive(Debug, Serialize)]
pub struct Checksums {
    checksums: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectOrDependency {
    #[serde(skip_serializing_if = "Option::is_none")]
    packageUri: Option<String>,
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    projectFileUri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checksums: Option<Checksums>,
    dependencies: HashMap<String, ProjectOrDependency>,
}

#[derive(Debug, Serialize)]
pub struct CreateEvaluator {
    pub requestId: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientResourceReaders: Option<Vec<ResourceReader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientModuleReaders: Option<Vec<ModuleReader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modulePaths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputFormat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowedModules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowedResources: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootDir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cacheDir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectOrDependency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeoutSeconds: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CloseEvaluator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluatorId: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct Evaluate {
    pub requestId: i64,
    pub evaluatorId: i64,
    pub moduleUri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moduleText: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReadResourceResponse {
    requestId: i64,
    evaluatorId: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReadModuleResponse {
    requestId: i64,
    evaluatorId: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListResourceResponse {
    requestId: i64,
    evaluatorId: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pathElements: Option<Vec<PathElement>>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListModulesResponse {
    requestId: i64,
    evaluatorId: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pathElements: Option<Vec<PathElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

        let mp = pack_message(OutgoingMessage::ListModulesResponse(mr)).unwrap();
        let expected = vec![0x92, 0x2D, 0x83, 0xA9, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x64, 0xD2, 0xFF, 0xF6, 0x1D, 0x2C, 0xAB, 0x65, 0x76, 0x61, 0x6C, 0x75, 0x61, 0x74, 0x6F, 0x72, 0x49, 0x64, 0xD1, 0xC9, 0xB3, 0xAC, 0x70, 0x61, 0x74, 0x68, 0x45, 0x6C, 0x65, 0x6D, 0x65, 0x6E, 0x74, 0x73, 0x91, 0x82, 0xA4, 0x6E, 0x61, 0x6D, 0x65, 0xA7, 0x66, 0x6F, 0x6F, 0x2E, 0x70, 0x6B, 0x6C, 0xAB, 0x69, 0x73, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x79, 0xC2];

        println!("Serialized: {:X?}", mp);

        assert_eq!(mp, expected);
    }
}
