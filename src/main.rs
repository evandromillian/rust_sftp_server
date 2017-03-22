extern crate regex;

use std::str;
use std::io::{ Write, BufRead, BufReader };
use std::thread;
use std::net::{ TcpListener, TcpStream };
//use std::collections::HashMap;
use regex::Regex;

mod status;

#[test]
fn test_regex() {
	let re = Regex::new(r"([A-Z]+)[\s]*([\w|.|/|\\|:]*)\r\n").unwrap();
	for cap in re.captures_iter("RETR /usr/file.txt\r\n") {
		assert_eq!("RETR", &cap[1]);
		assert_eq!("/usr/file.txt", &cap[2]);
	}
	for cap in re.captures_iter("QUIT\r\n") {
		assert_eq!("QUIT", &cap[1]);
	}
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:21").unwrap();
	for result in listener.incoming() {
		println!("Received client");
		match result {
			Ok(result_stream) => {
				thread::spawn(|| {
					let mut reader: BufReader<TcpStream> = BufReader::new(result_stream);
					let ftp_regex = Regex::new(r"([A-Z]+)[\s]*([\w|\d|\s|.|,|/|\\|:]*)\r\n").unwrap();
					
					// Welcome
					let _ = reader.get_mut().write(b"220 Rust FTP server\r\n");
					
					loop {
						let mut line = String::new();
						match reader.read_line(&mut line) {
							Ok(_) => {	
								println!("Command received: {}", line);							
								let caps = match ftp_regex.captures(line.as_str()) {
									Some(caps) => {
										caps
									}
									None => {
										println!("Client disconnected");
										break;
									}
								};
								let command = caps.get(1).unwrap().as_str();
								let arguments = caps.get(2).unwrap().as_str();
								
								match command {
									"OPTS" => {
										let _ = reader.get_mut().write(b"200 OK\r\n");
									}
									"USER" => {
										/**
											200 - OK
											331 - User name OK, need password
											332 -
											500, 501, 421
										*/
										let _ = reader.get_mut().write(b"331 User name OK, need password\r\n");
									}
									"PASS" => {
										/**
											230 - Logged in
											331 - User name OK, need password
											202
											530
											500, 501, 503, 421
											332
										*/
										let _ = reader.get_mut().write(b"230 Logged in\r\n");
									}
									"PORT" => {
										let v: Vec<&str> = arguments.split(',').collect();
										let ip = format!("{}.{}.{}.{}", v[0], v[1], v[2], v[3]);
										let hiPort: u32 = v[4].parse().unwrap();
										let lowPort: u32 = v[5].parse().unwrap();
										let port = format!("{:x}{:x}", hiPort, lowPort);
										let decPort = u32::from_str_radix(port.as_str(), 16).unwrap();
										println!("Client IP: {}", ip);
										println!("Client port: {}", decPort);

										let _ = reader.get_mut().write(b"502 Command not implemented\r\n");
									}
									"RETR" => {
										let _ = reader.get_mut().write(b"502 Command not implemented\r\n");
									}
									"QUIT" => {
										/**
											221 - OK
										*/
										let _ = reader.get_mut().write(b"221 Bye\r\n");
									}
									_ => {

										let _ = reader.get_mut().write(b"500 Command unrecognized\r\n");
									}
								}							
								
								
							}
							Err(err) => {
								println!("Error reading: {}", err);
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