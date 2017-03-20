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
	/*
	for cap in re.captures_iter("RETR /usr/file.txt\r\n") {
		assert_eq!("RETR", &cap[1]);
		assert_eq!("/usr/file.txt", &cap[2]);
	}
	*/
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
					//let mut stream = result_stream;
					let mut reader: BufReader<TcpStream> = BufReader::new(result_stream);
					let ftp_regex = Regex::new(r"([A-Z]+)[\s]*([\w|\s|.|/|\\|:]*)\r\n").unwrap();
					
					// Welcome
					let _ = reader.get_mut().write(b"220 Rust FTP server\r\n");
					
					loop {
						let mut line = String::new();
						match reader.read_line(&mut line) {
							Ok(_) => {								
								let caps = ftp_regex.captures(line.as_str()).unwrap();
								let command = caps.get(1).unwrap().as_str();
								let arguments = caps.get(2).unwrap().as_str();
								println!("Command: {}", &command);
								println!("Arguments: {}", &arguments);								
								
								
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