use std::io::{self, Write};

#[derive(Default)]
pub enum Logger {
    #[default] StderrLogger,
}

#[derive(Default)]
struct StderrLogger {}

trait LoggerImpl {
    fn trace(message: String, frame_uri: String) -> std::io::Result<()>;
    fn warn(message: String, frame_uri: String) -> std::io::Result<()>;
}

impl Logger {
    pub fn trace(&self, message: String, frame_uri: String) {
        match self {
            Logger::StderrLogger => StderrLogger::trace(message, frame_uri).expect("Encountered an IO error in StderrLogger::trace()"),
        }
    }

    pub fn warn(&self, message: String, frame_uri: String) {
        match self {
            Logger::StderrLogger => StderrLogger::warn(message, frame_uri).expect("Encountered an IO error in StderrLogger::warn()"),
        }
    }
}

impl LoggerImpl for StderrLogger {
    fn trace(message: String, frame_uri: String) -> std::io::Result<()> {
        let stderr = io::stderr();
        let mut handle = stderr.lock();

        let s: String = format!("TRACE: {} {}", message, frame_uri);

        write!(handle, "{}", s)?;
        Ok(())
    }

    fn warn(message: String, frame_uri: String) -> std::io::Result<()> {
        let stderr = io::stderr();
        let mut handle = stderr.lock();

        let s: String = format!("WARN: {} {}", message, frame_uri);

        write!(handle, "{}", s)?;
        Ok(())
    }
}
