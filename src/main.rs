extern crate mio;

use std::str;
use std::io::{Read, Write};
use std::thread;
use std::net::{TcpListener};
//use std::collections::HashMap;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:21").unwrap();
	for result in listener.incoming() {
		println!("Received client");
		match result {
			Ok(result_stream) => {
				thread::spawn(|| {
					let mut stream = result_stream;
					
					stream.write(b"220 Rust FTP server\r\n").unwrap();
					stream.flush().unwrap();
					
					loop {
						let mut cmd_buf = [0u8; 4];
						match stream.read_exact(&mut cmd_buf) {
							Ok(()) => {
								/**
									See list of FTP commands in 
									* https://en.wikipedia.org/wiki/List_of_FTP_commands
									* http://br.ccm.net/contents/265-o-protocolo-file-transfer-protocol
								*/
								let cmd = str::from_utf8(&cmd_buf).unwrap();
								match cmd {
									"OPTS" => {
										let mut arg_buf = [0u8; 32];
										match stream.read(&mut arg_buf) {
											Ok(_) => {
												let args = str::from_utf8(&arg_buf).unwrap();
												println!("Command {} args: {}", cmd, args);
												
												stream.write(b"200 OK\r\n").unwrap();
												stream.flush().unwrap();
											}
											Err(e) => {
												println!("Error answering {} command: {}", cmd, e);
												break;
											}
										}
									}
									"USER" => {
										let mut arg_buf = [0u8; 32];
										match stream.read(&mut arg_buf) {
											Ok(_) => {
												let args = str::from_utf8(&arg_buf).unwrap();
												println!("Command {} args: {}", cmd, args);
												
												stream.write(b"200 OK\r\n").unwrap();
												stream.flush().unwrap();
											}
											Err(e) => {
												println!("Error answering {} command: {}", cmd, e);
												break;
											}
										}
									}
									_ => { 
										println!("Command {} unrecognized", cmd);
										break;
									}
								}
								
							}
							Err(e) => { 
								println!("Error reading message: {}", e);
								break;
							}
						}
					}
				});
			}
			Err(e) => { 
				println!("Error accepting client: {}", e);
			}
		}
	}
}