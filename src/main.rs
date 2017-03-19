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
					
					// Welcome
					let _ = stream.write(b"220 Rust FTP server\r\n");
					let _ = stream.flush();
					
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
										let mut arg_buf = [0u8; 64];
										match stream.read(&mut arg_buf) {
											Ok(_) => {
												let args = str::from_utf8(&arg_buf).unwrap();
												println!("Command {} args: {}", cmd, args);
												
												let _ = stream.write(b"200 OK\r\n");
												let _ = stream.flush();
											}
											Err(e) => {
												println!("Error answering {} command: {}", cmd, e);
												break;
											}
										}
									}
									"USER" => {
										let mut arg_buf = [0u8; 128];
										match stream.read(&mut arg_buf) {
											Ok(_) => {
												let args = str::from_utf8(&arg_buf).unwrap();
												println!("Command {} args: {}", cmd, args);
												
												/**
													200 - OK
													331 - User name OK, need password
												*/
												let _ = stream.write(b"331 User name OK, need password\r\n");
												let _ = stream.flush();
											}
											Err(e) => {
												println!("Error answering {} command: {}", cmd, e);
												break;
											}
										}
									}
									"PASS" => {
										let mut arg_buf = [0u8; 128];
										match stream.read(&mut arg_buf) {
											Ok(_) => {
												let args = str::from_utf8(&arg_buf).unwrap();
												println!("Command {} args: {}", cmd, args);
												
												/**
													230 - Logged in
													331 - User name OK, need password
												*/
												let _ = stream.write(b"230 Logged in\r\n");
												let _ = stream.flush();
											}
											Err(e) => {
												println!("Error answering {} command: {}", cmd, e);
												break;
											}
										}
									}
									/*
									"PORT" => {

									}
									"TYPE" => {

									}
									"MODE" => {

									}
									"STRU" => {

									}
									"RETR" => {

									}
									"STOR" => {

									}
									*/
									"QUIT" => {
										let mut arg_buf = [0u8; 4];
										match stream.read(&mut arg_buf) {
											Ok(_) => {
												println!("Command {} ", cmd);
												
												/**
													221 - OK
												*/
												let _ = stream.write(b"221 Bye\r\n");
												let _ = stream.flush();
											}
											Err(e) => {
												println!("Error answering {} command: {}", cmd, e);
												break;
											}
										}
									}
									_ => { 
										println!("Command {} unrecognized", cmd);
										
										let mut arg_buf = [0u8; 128];
										let _ = stream.read(&mut arg_buf);
										let _ = stream.write(b"500 Command unrecognized\r\n");
										let _ = stream.flush();
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