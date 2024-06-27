/// Utility functions for the outgoing messages _from_ the
/// client (your program) to the server (pkl).

use std::collections::HashMap;

use rmp_serde as rmps;

use serde::Serialize;
use rmps::Serializer;

use super::code::{CODE_NEW_EVALUATOR,
                  CODE_NEW_EVALUATOR_RESPONSE,
                  CODE_CLOSE_EVALUATOR,
                  CODE_EVALUATE,
                  CODE_EVALUATE_RESPONSE,
                  CODE_EVALUATE_READ_RESPONSE,
                  CODE_EVALUATE_READ_MODULE_RESPONSE,
                  CODE_LIST_RESOURCES_RESPONSE,
                  CODE_LIST_MODULES_RESPONSE};

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
#[serde(rename_all = "camelCase")]
pub struct ModuleReader {
    pub scheme: String,
    pub has_hierarchical_uris: bool,
    pub is_globbable: bool,
    pub is_local: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceReader {
    pub scheme: String,
    pub has_hierarchical_uris: bool,
    pub is_globbable: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Checksums {
    pub checksums: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOrDependency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_uri: Option<String>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_file_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksums: Option<Checksums>,
    pub dependencies: HashMap<String, ProjectOrDependency>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEvaluator {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_resource_readers: Option<Vec<ResourceReader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_module_readers: Option<Vec<ModuleReader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_modules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_resources: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectOrDependency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseEvaluator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator_id: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Evaluate {
    pub request_id: i64,
    pub evaluator_id: i64,
    pub module_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadResourceResponse {
    pub request_id: i64,
    pub evaluator_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadModuleResponse {
    pub request_id: i64,
    pub evaluator_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourceResponse {
    pub request_id: i64,
    pub evaluator_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_elements: Option<Vec<PathElement>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModulesResponse {
    pub request_id: i64,
    pub evaluator_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_elements: Option<Vec<PathElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathElement {
    pub name: String,
    pub is_directory: bool,
}

#[cfg(test)]
mod tests {
    use crate::log;

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
        let pe = PathElement{name: "foo.pkl".into(), is_directory: false};
        let mr = ListModulesResponse{request_id: -647892, evaluator_id: -13901, path_elements: vec![pe].into(), error: None};

        let mp = pack_message(OutgoingMessage::ListModulesResponse(mr)).unwrap();
        let expected = vec![0x92, 0x2D, 0x83, 0xA9, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
                            0x74, 0x49, 0x64, 0xD2, 0xFF, 0xF6, 0x1D, 0x2C, 0xAB, 0x65,
                            0x76, 0x61, 0x6C, 0x75, 0x61, 0x74, 0x6F, 0x72, 0x49, 0x64,
                            0xD1, 0xC9, 0xB3, 0xAC, 0x70, 0x61, 0x74, 0x68, 0x45, 0x6C,
                            0x65, 0x6D, 0x65, 0x6E, 0x74, 0x73, 0x91, 0x82, 0xA4, 0x6E,
                            0x61, 0x6D, 0x65, 0xA7, 0x66, 0x6F, 0x6F, 0x2E, 0x70, 0x6B,
                            0x6C, 0xAB, 0x69, 0x73, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74,
                            0x6F, 0x72, 0x79, 0xC2];

        log!(1, "Serialized: {:X?}", mp);

        assert_eq!(mp, expected);
    }
}
