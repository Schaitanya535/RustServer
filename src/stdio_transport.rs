use std::io::{self, BufRead, BufReader, Read};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
/// A simple message transport for LSP that reads from stdin and processes messages
pub struct StdioTransport {
    reader: BufReader<io::Stdin>,
}

impl StdioTransport {
    /// Create a new stdio transport
    pub fn new() -> Self {
        StdioTransport {
            reader: BufReader::new(io::stdin()),
        }
    }

    /// Start reading LSP messages from stdin
    /// Returns a receiver that will provide the raw message content
    pub fn read_messages(self) -> Receiver<String> {
        let (tx, rx) = channel();
        let mut reader = self.reader;

        thread::spawn(move || {
            loop {
                // Read the content length header
                let mut header = String::new();
                match reader.read_line(&mut header) {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        if header.trim().is_empty() {
                            continue;
                        }

                        // Parse content length
                        let content_length = if header.starts_with("Content-Length: ") {
                            header["Content-Length: ".len()..]
                                .trim()
                                .parse::<usize>()
                                .unwrap_or(0)
                        } else {
                            eprintln!("Invalid header: {}", header);
                            continue;
                        };

                        // Skip the empty line after headers
                        let mut empty_line = String::new();
                        if reader.read_line(&mut empty_line).is_err() {
                            break;
                        }

                        // Read the message content
                        let mut content = vec![0; content_length];
                        if reader.read_exact(&mut content).is_err() {
                            break;
                        }

                        // Convert to string and send through the channel
                        match String::from_utf8(content) {
                            Ok(message) => {
                                if tx.send(message).is_err() {
                                    break;
                                }
                            }
                            Err(e) => eprintln!("Invalid UTF-8: {}", e),
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from stdin: {}", e);
                        break;
                    }
                }
            }
        });

        rx
    }
}
