use std::{sync::{atomic::AtomicBool, Mutex, mpsc::{Sender, Receiver, channel}}, collections::HashMap};

use super::{evaluator::Evaluator, msg_api::{incoming::IncomingMessage, outgoing::OutgoingMessage}};
use super::{msg_api::{incoming, outgoing}, evaluator_manager_exec::EvaluatorManagerExec};


#[derive(Default)]
pub struct EvaluatorManager {
    // interrupts: Mutex<HashMap<Sender<OutgoingMessage>, i64>>, // TODO https://docs.rs/async-map/latest/async_map/ ??
    evaluators: Evaluator,
    pending_evaluators: Evaluator,
    exec: EvaluatorManagerExec,
    closed: AtomicBool,
    initialized: bool,
}

impl EvaluatorManager {
    fn close() -> Result<&'static str, &'static str> {
        todo!()
    }

    fn get_version() -> Result<String, &'static str> {
        todo!()
    }

    fn new_evaluator<'a>() -> Result<Evaluator, &'static str> {
        todo!()
    }

    fn new_project_evaluator<'a>() -> Result<Evaluator, &'static str> {
        todo!()
    }
}
