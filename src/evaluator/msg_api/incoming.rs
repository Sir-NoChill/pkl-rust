#![allow(nonstandard_style)]

use std::collections::HashMap;

use rmp_serde as rmps;

use rmps::from_slice;
use serde::Deserialize;

pub fn decode<T: for<'a> Deserialize<'a>>(msg: Vec<u8>) -> Result<(u8, T), &'static str> {
    return Ok(from_slice::<(u8, T)>(&msg).expect("Failed to deserialize"));
}

#[derive(Deserialize, Debug, Clone)]
struct CreateEvaluatorResponse {
    requestId: i64,
    evaluatorId: Option<i64>, // if None, then error is Some(errmsg)
    error: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct EvaluateResponse {
    requestId: i64,
    evaluatorId: i64,
    result: Option<Vec<u8>>,
    error: Option<String>,
}


#[derive(Deserialize, Debug, Clone)]
struct ReadResource {
    requestId: i64,
    evaluatorId: i64,
    uri: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ReadModule {
    requestId: i64,
    evaluatorId: i64,
    uri: String
}

#[derive(Deserialize, Debug, Clone)]
struct ListResources {
    requestId: i64,
    evaluatorId: i64,
    uri: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ListModules {
    requestId: i64,
    evaluatorId: i64,
    uri: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Log {
    evaluatorId: i64,
    level: i8,
    message: String,
    frameUri: String,
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
