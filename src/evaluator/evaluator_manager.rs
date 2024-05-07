use std::{sync::{atomic::AtomicBool, Mutex, mpsc::{Sender, Receiver, channel}}, collections::HashMap, default, rc::Rc, thread, time::Duration};

use super::{evaluator::Evaluator, evaluator_options::EvaluatorOptions, msg_api::{incoming::IncomingMessage, outgoing::{OutgoingMessage, CreateEvaluator, pack_message}}};
use super::{msg_api::{incoming, outgoing}, evaluator_manager_exec::EvaluatorManagerExec};


#[derive(Default)]
pub struct EvaluatorManager {
    // interrupts: Mutex<HashMap<Sender<OutgoingMessage>, i64>>, // TODO https://docs.rs/async-map/latest/async_map/ ??
    evaluators: Vec<Evaluator>,
    // pending_evaluators: Evaluator,
    exec: EvaluatorManagerExec,
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

    pub fn new_evaluator(&self, options: Option<EvaluatorOptions>) -> Result<Evaluator, &'static str> {
        let mut evaluator = Evaluator::default();
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

        let message = pack_message(&message_data, OutgoingMessage::CreateEvaluator).expect("Failed to serialize message");

        thread::sleep(Duration::from_millis(100));
        let sender = self.exec.sender.clone();
        let _ = sender.send(message.clone());
        let eval_resp = match self.exec.receiver.recv_timeout(Duration::from_secs(3)).expect("Failed to receive eval message") {
            IncomingMessage::CreateEvaluatorResponse(x) => x,
            _ => return Err("Failed to get message"),
        };


        if eval_resp.error != None {
            return Err("PKL server issued an error in create evaluator");
        }

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

        return Ok(evaluator);
    }

    fn new_project_evaluator() -> Result<Evaluator, &'static str> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_evaluator() {
        let eval = EvaluatorManager::default();

        let _evaluator = eval.new_evaluator(None).expect("Failed to create a new evaluator");
    }

    #[test]
    fn test_close_evaluator() {

    }
}
