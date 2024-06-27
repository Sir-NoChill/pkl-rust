use std::{sync::mpsc::{Sender, channel}, any::Any, collections::HashMap};

use crate::evaluator::msg_api::incoming::IncomingMessage;

use super::{msg_api::{outgoing::{ResourceReader, ModuleReader}, incoming::EvaluateResponse},
            module_source::ModuleSource, evaluator_options::EvaluatorOptions};

// Interface for evaluating pkl modules
pub struct Evaluator { // NOTE the lifetime allows us to ignore close() since at the end of the lifetime the Evaluator is killed automatically
    pub evaluator_id: i64,
    // pub manager: Option<Rc<&EvaluatorManager>>, //TODO fix the bidirectional reference
    pub pending_requests: HashMap<i64, Sender<EvaluateResponse>>,
    pub closed: bool,
    pub resource_readers: Vec<ResourceReader>,
    pub module_readers: Vec<ModuleReader>,
    pub opts: EvaluatorOptions,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self {
            evaluator_id: rand::random(),
            // manager: Default::default(),
            pending_requests: Default::default(),
            closed: Default::default(),
            resource_readers: Default::default(),
            module_readers: Default::default(),
            opts: Default::default()
        }
    }
}

// TODO the `out` field should be replaced with some sort of
//  macro since we can evaluate what the type is at compile
//  time. //NOTE I'm dumb and wrong...
pub trait EvaluatorMethods { // NOTE this allows for other types of evaluators, which could be nice
    fn evaluate_module<T>(&self, source: &ModuleSource) -> Result<T, &'static str>;
    fn evaluate_output_text(&self, source: &ModuleSource) -> Result<&'static str, &'static str>;
    fn evaluate_output_value(&self, source: &ModuleSource, out: &dyn Any) -> Result<&'static str, &'static str>;
    fn evaluate_output_files(&self, source: &ModuleSource) -> Result<&'static str, &'static str>;
    fn evaluate_expression<T>(&self, source: &ModuleSource, expr: Option<String>) -> Result<T, &'static str>;
    fn evaluate_expression_raw<T>(&self, source: &ModuleSource, expr: Option<String>) -> Result<T, &'static str>;
    fn closed(&self, ) -> bool;
    fn close(&self);
}

impl EvaluatorMethods for Evaluator {
    fn evaluate_module<T>(&self, source: &ModuleSource) -> Result<T, &'static str> {
        return self.evaluate_expression(source, None);
    }

    fn evaluate_expression<T>(&self, source: &ModuleSource, expr: Option<String>) -> Result<T, &'static str> {
        return self.evaluate_expression_raw(source, expr);
    }

    fn evaluate_expression_raw<T>(&self, source: &ModuleSource, expr: Option<String>) -> Result<T, &'static str> {
        // let request_id: i64 = rand::random::<i64>();
        // let (send, recv) = channel::<IncomingMessage>();

        // let msg = Evaluate {
        //     request_id,
        //     evaluator_id: self.evaluator_id,
        //     module_uri: source.uri().to_string(),
        //     module_text: source.contents().clone(), //FIXME badness
        //     expr,
        // };
        todo!()
    }

    fn evaluate_output_text(&self, source: &ModuleSource) -> Result<&'static str, &'static str> {
        todo!()
    }

    fn evaluate_output_value(&self, source: &ModuleSource, out: &dyn Any) -> Result<&'static str, &'static str> {
        todo!()
    }

    fn evaluate_output_files(&self, source: &ModuleSource) -> Result<&'static str, &'static str> {
        todo!()
    }

    fn closed(&self, ) -> bool {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}
