
use std::path::PathBuf;

use serde::Deserialize;

use super::{evaluator::{Evaluator, EvaluatorMethods}, evaluator_options::EvaluatorOptions, msg_api::{incoming::IncomingMessage, outgoing::{OutgoingMessage, CreateEvaluator, pack_message, CloseEvaluator, Evaluate, ListModulesResponse, PathElement}}, module_source::ModuleSource};
use super::executor::Executor;


#[derive(Default)]
pub struct EvaluatorManager {
    // interrupts: Mutex<HashMap<Sender<OutgoingMessage>, i64>>, // TODO https://docs.rs/async-map/latest/async_map/ ??
    evaluators: Vec<Evaluator>,
    // pending_evaluators: Evaluator,
    exec: Executor,
    // closed: AtomicBool,
    // initialized: bool,
}

impl EvaluatorManager {
    fn close() -> Result<&'static str, &'static str> {
        todo!()
    }

    fn get_version() -> Result<String, &'static str> {
        todo!()
    }

     fn new_evaluator(&mut self, options: Option<EvaluatorOptions>) -> Result<i64, &'static str> {
        let opts = match options {
            None => Default::default(),
            Some(x) => x,
        };

        let message_data = CreateEvaluator {
            requestId: rand::random(),
            clientResourceReaders: None,
            clientModuleReaders: None,
            modulePaths: None,
            env: None,
            properties: None,
            outputFormat: None,
            allowedModules: Some(opts.allowed_modules.clone()),
            allowedResources: Some(opts.allowed_resources.clone()),
            rootDir: None,
            cacheDir: Some(opts.cache_dir.to_str().unwrap().to_string()), //TODO error
            project: None,
            timeoutSeconds: None,
        };

        let eval_resp = match self.exec.senrec(OutgoingMessage::CreateEvaluator(message_data)).expect("Failed to send message") {
            IncomingMessage::CreateEvaluatorResponse(x) => x,
            _ => panic!("Unexpected response"),
        };

        let evaluator = Evaluator {
            evaluator_id: eval_resp.evaluatorId.unwrap(), // if we did not error, then this is guaranteed
            logger: Default::default(),
            // manager: Some(Rc::new(self)), // FIXME see Evaluator.rs
            pending_requests: Default::default(),
            closed: false,
            resource_readers: Default::default(),
            module_readers: Default::default(),
            opts,
        };

        let res = evaluator.evaluator_id.clone();

        self.evaluators.push(evaluator);

        return Ok(res);
    }

    fn new_project_evaluator() -> Result<Evaluator, &'static str> {
        todo!()
    }

    fn evaluate_module<T: for<'a> Deserialize<'a>>(&mut self, file: String, id_number: i64) -> Result<T, &'static str> {
        // send the evaluate request
        let eval_req = Evaluate {
            requestId: rand::random::<i64>(),
            evaluatorId: id_number,
            moduleUri: file.clone(),
            moduleText: None,
            expr: None,
        };

        let eval_msg = OutgoingMessage::Evaluate(eval_req);
        let mut resp = self.exec.senrec(eval_msg).expect("Failed to receive message");

        loop {
            match &mut resp {
                IncomingMessage::EvaluateResponse(x) => {
                    let close_msg = CloseEvaluator {
                        evaluatorId: Some(id_number.clone()),
                    };

                    self.exec.send(OutgoingMessage::CloseEvaluator(close_msg));
                    //TODO get the data and decode

                    let data = x.clone().result.expect("failed to get result");

                    // FIXME fails to decode, need to unmarshal data
                    print!("Data: ");
                    for d in &data {
                        print!("{:#04X}, ", d);
                    }
                    println!();

                    #[derive(Deserialize, Debug)]
                    struct PklEncoding {
                        v: i64,
                        module: String,
                        file: String,
                        data: Vec<u8>,
                    }
                    let res: PklEncoding = rmp_serde::from_slice(&data).expect("Failed to deserialize");
                    println!("Res: {:?}", res);
                    return Err("e");
                },
                IncomingMessage::ReadResource(x) => todo!(),
                IncomingMessage::ReadModule(x) => todo!(),
                IncomingMessage::ListResources(x) => todo!(),
                IncomingMessage::ListModules(x) => {
                    // get all the files in the module:
                    let path = PathBuf::from(file.clone());
                    // let mut files = file;
                    if path.is_dir() {
                        // files = std::fs::read_dir(path); // TODO
                    }

                    let mut modules: Vec<PathElement> = vec![];
                    // for file in files {
                    //     // TODO make module
                    // }

                    let list_resp = ListModulesResponse{
                        requestId: x.requestId,
                        evaluatorId: id_number.clone(),
                        pathElements: Some(modules),
                        error: None,
                    };

                    let resp = self.exec.senrec(OutgoingMessage::ListModulesResponse(list_resp)).expect("Failed to send/receive data");

                },
                IncomingMessage::Log(_) => todo!(),
                _ => return Err("Client received unexpected response from server"),
            }
        }

        // send the any required list_moduels response
        // send any read_module_response
        // send the close evaluator
    }
}

impl Drop for EvaluatorManager {
    fn drop(&mut self) {
        for evaluator in &self.evaluators {
            let msg = CloseEvaluator {
                evaluatorId: Some(evaluator.evaluator_id),
            };
            self.exec.send(OutgoingMessage::CloseEvaluator(msg));
            // evaluator.close();
        }

        // Droppign the EvaluatorManagerExec is automatic
        //  from the drop trait
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn test_new_evaluator() {
        let mut eval = EvaluatorManager::default();

        let _evaluator = eval.new_evaluator(None).expect("Failed to create a new evaluator");
    }

    #[test]
    fn test_standard_pipeline() {
        #[derive(Deserialize)]
        struct Test {
            foo: i64,
            bar: i32,
        }

        let mut eval = EvaluatorManager::default();

        let evaluator = eval.new_evaluator(None).expect("Failed to create a new evaluator");

        let test: Test = eval.evaluate_module::<Test>("file:///home/stormblessed/Code/pkl-rust/src/evaluator/tests/test.pkl".into(), evaluator).expect("Failed to obtain result");

        assert_eq!(test.foo, 1);
        assert_eq!(test.bar, 2);
    }
}
