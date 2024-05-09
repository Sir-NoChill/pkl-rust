
use super::{evaluator::{Evaluator, EvaluatorMethods}, evaluator_options::EvaluatorOptions, msg_api::{incoming::IncomingMessage, outgoing::{OutgoingMessage, CreateEvaluator, pack_message, CloseEvaluator}}};
use super::evaluator_manager_exec::EvaluatorManagerExec;


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

    pub fn new_evaluator(&mut self, options: Option<EvaluatorOptions>) -> Result<Evaluator, &'static str> {
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

        return Ok(evaluator);
    }

    fn new_project_evaluator() -> Result<Evaluator, &'static str> {
        todo!()
    }
}

impl Drop for EvaluatorManager {
    fn drop(&mut self) {
        for evaluator in &self.evaluators {
            let msg = CloseEvaluator {
                evaluatorId: Some(evaluator.evaluator_id),
            };
            self.exec.send(OutgoingMessage::CloseEvaluator(msg));
            evaluator.close();
        }

        // Droppign the EvaluatorManagerExec is automatic
        //  from the drop trait
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_evaluator() {
        let mut eval = EvaluatorManager::default();

        let _evaluator = eval.new_evaluator(None).expect("Failed to create a new evaluator");
    }

    #[test]
    fn test_close_evaluator() {
    }
}
