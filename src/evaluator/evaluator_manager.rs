use std::{sync::{atomic::AtomicBool, Mutex, mpsc::{Sender, Receiver, channel}}, collections::HashMap};

use super::{evaluator::Evaluator, msg_api::{incoming::IncomingMessage, outgoing::OutgoingMessage}};
use super::{msg_api::{incoming, outgoing}, evaluator_manager_exec::EvaluatorManagerExec};


#[derive(Default)]
pub struct EvaluatorManager<'a> {
    interrupts: Mutex<HashMap<Sender<OutgoingMessage>, i64>>, // TODO https://docs.rs/async-map/latest/async_map/ ??
    evaluators: Mutex<Vec<Evaluator<'a>>>,
    pending_evaluators: Mutex<Vec<Evaluator<'a>>>,
    exec: Vec<EvaluatorManagerExec>,
    closed: AtomicBool,
    new_evaluator_mutex: Mutex<u8>,
    initialized: bool,
}

impl EvaluatorManager<'_> {
    fn close() -> Result<&'static str, &'static str> {
        todo!()
    }

    fn get_version() -> Result<String, &'static str> {
        todo!()
    }

    fn new_evaluator<'a>() -> Result<Evaluator<'a>, &'static str> {
        todo!()
    }

    fn new_project_evaluator<'a>() -> Result<Evaluator<'a>, &'static str> {
        todo!()
    }
}
