use super::evaluator::Evaluator;

fn new_evaluator<'a>() -> Result<Evaluator<'a>, &'static str> {
    todo!()
}

fn new_project_evaluator<'a>(project_dir: String) -> Result<Evaluator<'a>, &'static str> {
    todo!()
}

fn new_project_evaluator_with_command<'a>(project_dir: String, pkl_command: Vec<String>) -> Result<Evaluator<'a>, &'static str> {
    todo!()
}

fn new_evaluator_with_command<'a>(pkl_command: Vec<String>) -> Result<Evaluator<'a>, &'static str> {
    todo!()
}
