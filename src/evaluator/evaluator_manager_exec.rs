use std::{process::{Command, Child, Stdio, ChildStdin, ChildStdout}, sync::{mpsc::{Sender, Receiver, channel, RecvError}, atomic::AtomicBool}, os::unix::process::CommandExt, io::{Write, BufReader, BufWriter, Read}, cmp::Reverse, thread::{self, JoinHandle}, time::{Duration, Instant}};

use serde::Serialize;

use super::msg_api::{incoming::*, outgoing::*, code::*};

/// A struct that handles the communication with the pkl evaluator
///
/// This is essentially a wrapper to hold and abstract the
/// spawned pkl child process
///
/// # Diagram
///
/// When we instantiate an EvaluatorManagerExec, we spawn a pkl process
/// and then communicate to it via message passing with two threads
/// handling the incoming and outgoing messages respectively.
///```no-run
///   pkl-rust           pkl
///      |  get version
///      |--------------->|
///      |<---------------|
///      |
///      | spawn server
///      |--------------->| // we store the channels here
///      |<---------------|
///      |                |
///      | pass messages  |
///      |  ...           |
///      |                |
///      | close server   |
///      |--------------->| // in theory the close is unidirectional
///      |
///      | kill thread is automatic
///      |
///      ...
///      user program
///```
///
#[derive(Debug)]
pub struct EvaluatorManagerExec {
    child_process: Child,
    pub version: String,
    pub pkl_command: Vec<String>,
    pub child_out: Option<ChildStdout>,
    pub child_in: Option<ChildStdin>,
}

impl Default for EvaluatorManagerExec {
    fn default() -> Self {
        //TODO need to get the pkl command from the user? or at least search the system PATH
        let pkl_command = vec!["/home/stormblessed/software/pkl".to_string(), "server".to_string()];

        // Checking the version of pkl on the host
        let version_check = Command::new(pkl_command.first().expect("Lol, well that sucked").to_string())
                                .arg("--version")
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("Failed to start pkl process");

        let v_out = version_check.wait_with_output().expect("Failed to get output").stdout;
        let version: String = String::from_utf8(v_out.to_vec()).expect("Failed to convert output to string");

        // Init the actual child process
        let mut child_process = Command::new(pkl_command.first().expect("no pkl command given").to_string())
                                .args(pkl_command.split_first().expect("pkl_command vector is empty!").1)
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .stderr(Stdio::piped())
                                .spawn()
                                .expect("failed to spawn pkl server process");

        // Thread spawning
        let child_in: ChildStdin = child_process.stdin.take().expect("Failed to open stdout");
        let child_out: ChildStdout = child_process.stdout.take().expect("Failed to open stdin");
        // let _ = spawn_write_thread(t_recv, kill_recv_w, child_in);
        // let _ = spawn_read_thread(t_send, kill_recv_r, child_out);

        Self {
            version,
            pkl_command,
            child_process,
            child_in: Some(child_in),
            child_out: Some(child_out),
        }
    }
}

impl EvaluatorManagerExec {
    /// Internal method to kill the evaluator
    fn deinit(&mut self) -> Result<(), std::io::Error> {
        //TODO this should also be logged
        // self.killer_recv.send(vec![0xff, 0xff]).expect("Failed to send kill signal to pkl reader");
        // self.killer_wrtr.send(vec![0xff, 0xff]).expect("Failed to send kill signal to pkl writer");

        self.child_process.kill()
    }

    // REVIEW: is it possible to make these async?
    pub(crate) fn send(&mut self, msg: OutgoingMessage) {
        let message: Vec<u8> = pack_message(msg).expect("Failed to pack message");

        self.child_in.take().expect("failed to take").write_all(&message).expect("Failed to send message");
        // println!("Sent message: {:?}", msg);
    }

    pub(crate) fn senrec(&mut self, msg: OutgoingMessage) -> Result<IncomingMessage, RecvError> {
        self.send(msg);

        let mut buf = [0u8; 2];
        let prefix: MessageCode;
        // now the fun part
        let mut out = self.child_out.take().expect("Failed to take"); // need to take here to use in the match
        out.read_exact(&mut buf).expect("Failed to read buffer");

        prefix = MessageCode::try_from(buf[1]).expect("Failed to convert to MessageCode");

        let mut value: Option<IncomingMessage> = None;
        // TODO not very DRY, but this might be the most idiomatic way to use serde
        match prefix {
            MessageCode::NewEvaluatorResponse => {
                println!("Matched new evaluator, Code: {:02X?}", prefix); //TODO Switch to logging
                match rmp_serde::from_read::<_, CreateEvaluatorResponse>(&mut out) {
                    Ok(msg) => value = Some(IncomingMessage::CreateEvaluatorResponse(msg)),
                    Err(err) => eprintln!("Error decoding the message: {}", err),
                }
            },
            MessageCode::EvaluateResponse => {
                println!("Matched new evaluator, Code: {:02X?}", prefix);
                match rmp_serde::from_read::<_, EvaluateResponse>(&mut out) {
                    Ok(msg) => value = Some(IncomingMessage::EvaluateResponse(msg)),
                    Err(err) => eprintln!("Error decoding the message: {}", err),
                }
            },
            MessageCode::EvaluateReadModule => {
                println!("Matched new evaluator, Code: {:02X?}", prefix);
                match rmp_serde::from_read::<_, ReadModule>(&mut out) {
                    Ok(msg) => value = Some(IncomingMessage::ReadModule(msg)),
                    Err(err) => eprintln!("Error decoding the message: {}", err),
                }
            },
            MessageCode::ListResourcesRequest => {
                println!("Matched new evaluator, Code: {:02X?}", prefix);
                match rmp_serde::from_read::<_, ListResources>(&mut out) {
                    Ok(msg) => value = Some(IncomingMessage::ListResources(msg)),
                    Err(err) => eprintln!("Error decoding the message: {}", err),
                }
            },
            MessageCode::ListModulesRequest => {
                println!("Matched new evaluator, Code: {:02X?}", prefix);
                match rmp_serde::from_read::<_, ListModules>(&mut out) {
                    Ok(msg) => value = Some(IncomingMessage::ListModules(msg)),
                    Err(err) => eprintln!("Error decoding the message: {}", err),
                }
            },
            MessageCode::EvaluateLog => {
                println!("Matched new evaluator, Code: {:02X?}", prefix);
                match rmp_serde::from_read::<_, Log>(&mut out) {
                    Ok(msg) => value = Some(IncomingMessage::Log(msg)),
                    Err(err) => eprintln!("Error decoding the message: {}", err),
                }
            },
            _ => {
                return Err(RecvError)
            }
        }
        return Ok(value.expect("Failed to retrieve value"));
    }

    // fn send(&self, msg: &impl Serialize, t: OutgoingMessage) {
    //     let message = pack_message(msg, t).expect("Could not determine message type, failed to serialize");
    //     self.sender.send(message).expect("Failed to send message to pkl");
    // }

    // fn recv(&self) -> Result<IncomingMessage, RecvError> {
    //     self.receiver.recv()
    // }
}

impl Drop for EvaluatorManagerExec {
    fn drop(&mut self) {
        let _ = self.deinit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn print_binary(vec: &[u8]) {
        print!("Binary       : ");
        print!("[");
        for i in vec {
            print!("0x{i:02X}, ");
        }
        println!("]");

    }

    #[test]
    fn test_regular_send() {
        let mut eval: EvaluatorManagerExec = EvaluatorManagerExec::default();

        let mut r = [0u8;2];

        //TODO extract these as constants
        let allowed_modules: Vec<String> = vec!["pkl:".into(), "repl:".into(), "file:".into(), "customfs:".into()];
        let resource_reader = vec![ResourceReader {
            scheme: "customfs".into(),
            hasHierarchicalUris: true,
            isGlobbable: true,
        }];

        let create_eval = CreateEvaluator {
            requestId: 135,
            clientResourceReaders: Some(resource_reader),
            allowedModules: Some(allowed_modules),
            clientModuleReaders: None,
            modulePaths: None,
            env: None,
            properties: None,
            outputFormat: None,
            allowedResources: None,
            rootDir: None,
            cacheDir: None,
            project: None,
            timeoutSeconds: None,
        };

        let test1 = pack_message(OutgoingMessage::CreateEvaluator(create_eval)).expect("Failed to pack");

        let _ = eval.child_in.take().unwrap().write(&test1.to_vec());
        // println!("Wrote message: {:?}", &test1.to_vec());
        // print_binary(&test1.to_vec());
        let a = eval.child_out.take().unwrap().read_exact(&mut r);
        // print_binary(&r);
        assert!(a.is_ok());
        assert!(r[1] == 33);
    }

    #[test]
    fn test_senrec() {
        let mut eval = EvaluatorManagerExec::default();

        let allowed_modules: Vec<String> = vec!["pkl:".into(), "repl:".into(), "file:".into(), "customfs:".into()];
        let resource_reader = vec![ResourceReader {
            scheme: "customfs".into(),
            hasHierarchicalUris: true,
            isGlobbable: true,
        }];

        let create_eval = CreateEvaluator {
            requestId: 135,
            clientResourceReaders: Some(resource_reader),
            allowedModules: Some(allowed_modules),
            clientModuleReaders: None,
            modulePaths: None,
            env: None,
            properties: None,
            outputFormat: None,
            allowedResources: None,
            rootDir: None,
            cacheDir: None,
            project: None,
            timeoutSeconds: None,
        };

        // print_binary(&pack_message(OutgoingMessage::CreateEvaluator(create_eval)).expect("failed"));
        // print_binary(&test1);

        let result = eval.senrec(OutgoingMessage::CreateEvaluator(create_eval)).expect("Failed to accept");
        println!("Received evaluator response: {:?}", result);
    }

}
