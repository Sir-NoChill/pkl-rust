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
///      | kill thread
///      |
///      ...
///      user program
///```
///
pub struct EvaluatorManagerExec {
    child_process: Child,
    pub sender: Sender<Vec<u8>>,
    pub receiver: Receiver<IncomingMessage>,
    killer_recv: Sender<Vec<u8>>,
    killer_wrtr: Sender<Vec<u8>>,
    pub version: String,
    pub pkl_command: Vec<String>,
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

        // Get our channels for communicating values
        let (sender, t_recv): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
        let (kill_sender_r, kill_recv_r): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
        let (kill_sender_w, kill_recv_w): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
        let (t_send, receiver): (Sender<IncomingMessage>, Receiver<IncomingMessage>) = channel();

        // Thread spawning
        let child_in: ChildStdin = child_process.stdin.take().expect("Failed to open stdout");
        let child_out: ChildStdout = child_process.stdout.take().expect("Failed to open stdin");
        let _ = spawn_write_thread(t_recv, kill_recv_w, child_in);
        let _ = spawn_read_thread(t_send, kill_recv_r, child_out);

        Self {
            version,
            sender,
            receiver,
            killer_recv: kill_sender_r,
            killer_wrtr: kill_sender_w,
            pkl_command,
            child_process,
        }
    }
}

impl EvaluatorManagerExec {
    /// Internal method to kill the evaluator
    fn deinit(&mut self) -> Result<(), std::io::Error> {
        //TODO this should also be logged
        self.killer_recv.send(vec![0xff, 0xff]).expect("Failed to send kill signal to pkl reader");
        self.killer_wrtr.send(vec![0xff, 0xff]).expect("Failed to send kill signal to pkl writer");

        self.child_process.kill()
    }

    //FIXME none of these methods work, so eliminating them
    // fn send_and_rec(&mut self, msg: &impl Serialize, t: OutgoingMessage) -> Result<IncomingMessage, RecvError> {
    //     let message = pack_message(msg, t).expect("Failed to pack message");

    //     self.sender.send(message).expect("Failed to send message");
    //     thread::sleep(Duration::from_millis(10));
    //     self.receiver.recv()
    // }

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

fn spawn_write_thread(recv: Receiver<Vec<u8>>, kill_recv: Receiver<Vec<u8>>, child_in: ChildStdin) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut writer = BufWriter::new(child_in); // Need the stream in order to write
        let mut message: Vec<u8>;
        loop {
            match kill_recv.recv_timeout(Duration::from_millis(10)) {
                Err(x) => println!("No message (s): {:?}", x),
                Ok(..) => {
                    println!("Received kill message (s)");
                    break;
                },
            }

            match recv.recv() {
                Ok(x) => message = x,
                Err(..) => continue,
            }

            match writer.write_all(&message) {
                Ok(_) => {
                    if let Err(err) = writer.flush() {
                        eprintln!("Error flushing: {}", err);
                    }
                },
                Err(err) => eprintln!("Error serializing message: {}", err), //TODO this should be logged instead
            }
            println!("Wrote message {:?}", &message);
        }
    })
}

fn spawn_read_thread<'f>(send: Sender<IncomingMessage>, recv: Receiver<Vec<u8>>, child_out: ChildStdout) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut reader = BufReader::new(child_out); //TODO the raw pointer is kind of inelegant
        let mut byte_prefix = [0u8; 2];

        loop {
            match recv.recv_timeout(Duration::from_millis(100)) {
                Err(x) => println!("No message (r): {:?}", x),
                Ok(..) => {
                    println!("Received kill message (r)");
                    break;
                },
            }

            match reader.read_exact(&mut byte_prefix) {
                Ok(..) => thread::sleep(Duration::from_millis(100)),
                Err(..) => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                },
            }
            println!("Recieved message: {:?}", &byte_prefix);

            let prefix = MessageCode::try_from(byte_prefix[1]).expect("Failed to resolve message type");
            let mut value: Option<IncomingMessage> = None;
            println!("Recieved message: {:?}", value);

            // TODO not very DRY, but this might be the most idiomatic way to use serde
            match prefix {
                MessageCode::NewEvaluatorResponse => {
                    println!("Matched new evaluator, Code: {:02X?}", prefix);
                    match rmp_serde::from_read::<_, CreateEvaluatorResponse>(&mut reader) {
                        Ok(msg) => value = Some(IncomingMessage::CreateEvaluatorResponse(msg)),
                        Err(err) => eprintln!("Error decoding the message: {}", err),
                    }
                },
                MessageCode::EvaluateResponse => {
                    println!("Matched new evaluator, Code: {:02X?}", prefix);
                    match rmp_serde::from_read::<_, EvaluateResponse>(&mut reader) {
                        Ok(msg) => value = Some(IncomingMessage::EvaluateResponse(msg)),
                        Err(err) => eprintln!("Error decoding the message: {}", err),
                    }
                },
                MessageCode::EvaluateReadModule => {
                    println!("Matched new evaluator, Code: {:02X?}", prefix);
                    match rmp_serde::from_read::<_, ReadModule>(&mut reader) {
                        Ok(msg) => value = Some(IncomingMessage::ReadModule(msg)),
                        Err(err) => eprintln!("Error decoding the message: {}", err),
                    }
                },
                MessageCode::ListResourcesRequest => {
                    println!("Matched new evaluator, Code: {:02X?}", prefix);
                    match rmp_serde::from_read::<_, ListResources>(&mut reader) {
                        Ok(msg) => value = Some(IncomingMessage::ListResources(msg)),
                        Err(err) => eprintln!("Error decoding the message: {}", err),
                    }
                },
                MessageCode::ListModulesRequest => {
                    println!("Matched new evaluator, Code: {:02X?}", prefix);
                    match rmp_serde::from_read::<_, ListModules>(&mut reader) {
                        Ok(msg) => value = Some(IncomingMessage::ListModules(msg)),
                        Err(err) => eprintln!("Error decoding the message: {}", err),
                    }
                },
                MessageCode::EvaluateLog => {
                    println!("Matched new evaluator, Code: {:02X?}", prefix);
                    match rmp_serde::from_read::<_, Log>(&mut reader) {
                        Ok(msg) => value = Some(IncomingMessage::Log(msg)),
                        Err(err) => eprintln!("Error decoding the message: {}", err),
                    }
                },
                _ => {
                    panic!("Failed to match any code {:?}", prefix);
                }
            }

            send.send(value.expect("Failed to deserialize the message")).expect("Failed to send result");
        }
    })
}

#[cfg(test)]
mod tests {
    use std::process::Stdio;


    use super::*;
    const test1: [u8; 139] = [0x92, 0x20, 0x83, 0xA9, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
                         0x74, 0x49, 0x64, 0xCC, 0x87, 0xAE, 0x61, 0x6C, 0x6C, 0x6F,
                         0x77, 0x65, 0x64, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x73,
                         0x94, 0xA4, 0x70, 0x6B, 0x6C, 0x3A, 0xA5, 0x72, 0x65, 0x70,
                         0x6C, 0x3A, 0xA5, 0x66, 0x69, 0x6C, 0x65, 0x3A, 0xA9, 0x63,
                         0x75, 0x73, 0x74, 0x6F, 0x6D, 0x66, 0x73, 0x3A, 0xB3, 0x63,
                         0x6C, 0x69, 0x65, 0x6E, 0x74, 0x4D, 0x6F, 0x64, 0x75, 0x6C,
                         0x65, 0x52, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x91, 0x84,
                         0xA6, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x65, 0xA8, 0x63, 0x75,
                         0x73, 0x74, 0x6F, 0x6D, 0x66, 0x73, 0xB3, 0x68, 0x61, 0x73,
                         0x48, 0x69, 0x65, 0x72, 0x61, 0x72, 0x63, 0x68, 0x69, 0x63,
                         0x61, 0x6C, 0x55, 0x72, 0x69, 0x73, 0xC3, 0xAB, 0x69, 0x73,
                         0x47, 0x6C, 0x6F, 0x62, 0x62, 0x61, 0x62, 0x6C, 0x65, 0xC3,
                         0xA7, 0x69, 0x73, 0x4C, 0x6F, 0x63, 0x61, 0x6C, 0xC3];

    const CREATE_EVAL: CreateEvaluator = CreateEvaluator{
        requestId: 140,
        clientResourceReaders: None,
        clientModuleReaders: None,
        modulePaths: None,
        env: None,
        properties: None,
        outputFormat: None,
        allowedModules: None,
        allowedResources: None,
        rootDir: None,
        cacheDir: None,
        project: None,
        timeoutSeconds: None,
    };

    #[test]
    fn test_pub() {
        let mut eval = EvaluatorManagerExec::default();

        let _ = &eval.sender.send(test1.to_vec());
        let a = &eval.receiver.recv();
        println!("Result: {:?}", a);

        let _ = &eval.sender.send(test1.to_vec());
        let a = &eval.receiver.recv();
        println!("Result: {:?}", a);

        let _ = &eval.sender.send(test1.to_vec());
        let a = &eval.receiver.recv();
        println!("Result: {:?}", a);

        let _ = &eval.sender.send(test1.to_vec());
        let a = &eval.receiver.recv();
        println!("Last one: {:?}", a);
    }

}
