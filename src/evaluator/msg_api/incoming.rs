#![allow(nonstandard_style)]

use rmp_serde as rmps;

use rmps::from_slice;
use serde::Deserialize;

pub fn decode<T: for<'a> Deserialize<'a>>(msg: Vec<u8>) -> Result<(u8, T), &'static str> {
    return Ok(from_slice::<(u8, T)>(&msg).expect("Failed to deserialize"));
}

pub trait DeserializableMessage {
    fn deserialize(reader: &mut dyn std::io::Read) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum IncomingMessage {
    CreateEvaluatorResponse(CreateEvaluatorResponse),
    EvaluateResponse(EvaluateResponse),
    ReadResource(ReadResource),
    ReadModule(ReadModule),
    ListResources(ListResources),
    ListModules(ListModules),
    Log(Log),
}

impl IncomingMessage {
    fn create_evaluator_response(&self) -> Option<CreateEvaluatorResponse> {
        if let IncomingMessage::CreateEvaluatorResponse(msg) = self {
            Some(msg.clone())
        } else {
            None
        }
    }
    fn evaluate_response(&self) -> Option<EvaluateResponse> {
        if let IncomingMessage::EvaluateResponse(msg) = self {
            Some(msg.clone())
        } else {
            None
        }
    }
    fn read_resource(&self) -> Option<ReadResource> {
        if let IncomingMessage::ReadResource(msg) = self {
            Some(msg.clone())
        } else {
            None
        }
    }
    fn read_module(&self) -> Option<ReadModule> {
        if let IncomingMessage::ReadModule(msg) = self {
            Some(msg.clone())
        } else {
            None
        }
    }
    fn list_resources(&self) -> Option<ListResources> {
        if let IncomingMessage::ListResources(msg) = self {
            Some(msg.clone())
        } else {
            None
        }
    }
    fn list_modules(&self) -> Option<ListModules> {
        if let IncomingMessage::ListModules(msg) = self {
            Some(msg.clone())
        } else {
            None
        }
    }
    fn log(&self) -> Option<Log> {
        if let IncomingMessage::Log(msg) = self {
            Some(msg.clone())
        } else {
            None
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateEvaluatorResponse {
    pub requestId: i64,
    pub evaluatorId: Option<i64>, // if None, then error is Some(errmsg)
    pub error: Option<String>,
}
impl DeserializableMessage for CreateEvaluatorResponse {
    fn deserialize(reader: &mut dyn std::io::Read) -> Option<Self>
    where
        Self: Sized {
        match rmp_serde::from_read(reader) {
            Ok(msg) => Some(msg),
            Err(_) => None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct EvaluateResponse {
    pub requestId: i64,
    pub evaluatorId: i64,
    pub result: Option<Vec<u8>>,
    pub error: Option<String>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct ReadResource {
    pub requestId: i64,
    pub evaluatorId: i64,
    pub uri: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReadModule {
    pub requestId: i64,
    pub evaluatorId: i64,
    pub uri: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListResources {
    pub requestId: i64,
    pub evaluatorId: i64,
    pub uri: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListModules {
    pub requestId: i64,
    pub evaluatorId: i64,
    pub uri: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Log {
    pub evaluatorId: i64,
    pub level: i8,
    pub message: String,
    pub frameUri: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_tuple() {
        // [ 33, {"requestId": 135, "evaluatorId": -135901, "error": null }]
        let eval_response: Vec<u8> = vec![0x92, 0x21, 0x83, 0xA9, 0x72, 0x65, 0x71, 0x75, 0x65,
                                 0x73, 0x74, 0x49, 0x64, 0xCC, 0x87, 0xAB, 0x65, 0x76,
                                 0x61, 0x6C, 0x75, 0x61, 0x74, 0x6F, 0x72, 0x49, 0x64,
                                 0xD2, 0xFF, 0xFD, 0xED, 0x23, 0xA5, 0x65, 0x72, 0x72,
                                 0x6F, 0x72, 0xC0];
        let res: (u8, CreateEvaluatorResponse) = rmps::from_slice(&eval_response).unwrap();
        let code = res.0.clone();
        let elem = res.1.clone();

        println!("Result of Deserialization: {:?}", res);
        assert_eq!(code, 0x21);
        assert_eq!(elem.requestId, 135);
        assert_eq!(elem.evaluatorId, Some(-135901));
        assert_eq!(elem.error, None);

        let eval_null = vec![0x92, 0x21, 0x82, 0xA9, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
                             0x74, 0x49, 0x64, 0xCC, 0x87, 0xAB, 0x65, 0x76, 0x61, 0x6C,
                             0x75, 0x61, 0x74, 0x6F, 0x72, 0x49, 0x64, 0xD2, 0xFF, 0xFD,
                             0xED, 0x23];

        // [ 33, {"requestId": 135, "evaluatorId": -135901 }] //note the ommissiono of the null field
        let res: (u8, CreateEvaluatorResponse) = rmps::from_slice(&eval_null).unwrap();
        let code2 = res.0.clone();
        let elem2 = res.1.clone();

        println!("Result of Deserialization: {:?}", res);
        assert_eq!(code2, 0x21);
        assert_eq!(elem2.requestId, 135);
        assert_eq!(elem2.evaluatorId, Some(-135901));
        assert_eq!(elem2.error, None);
    }
}
