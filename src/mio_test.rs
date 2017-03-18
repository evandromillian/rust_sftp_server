/**
    See tutorial in http://carllerche.github.io/mio/mio/struct.Poll.html
*/
/*

#[macro_use] extern crate text_io;

#[test]
fn test_scan() {
	let mut cmd: String;
	let mut val: String;
    let mut it = "USER evandro PASS evan123".bytes();
	//scan!("USER evandro PASS evan123".bytes() => "{} ", cmd);
    scan!(it => "{} {}", cmd, val);
    //scan!(it => "{} ", val);
	assert_eq!("USER", cmd);
    assert_eq!("evandro", val);
    scan!(it => "{} ", cmd, val);
    //scan!(it => "{} ", val);
    assert_eq!("PASS", cmd);
    assert_eq!("evan123", val);
}

fn old_main() {

    let mut connsMap : HashMap<Token, mio::tcp::TcpStream> = HashMap::new();
    let mut counter = 1;

    let poll = Poll::new().unwrap();                    // Create an poll instance

    let mut events = Events::with_capacity(1024);       // Create storage for events

    let addr = "127.0.0.1:13265".parse().unwrap();      // Setup the server socket
    let server = TcpListener::bind(&addr).unwrap();    

    poll.register(&server,                              // Start listening for incoming connections
                  SERVER, 
                  Ready::readable(), 
                  PollOpt::edge()).unwrap();

    let mut sock = TcpStream::connect(&addr).unwrap();  // Setup the client socket
    let tok = Token(counter);
    poll.register(&sock, 
                  tok, 
                  Ready::readable(), 
                  PollOpt::edge() /*| PollOpt::oneshot()*/).unwrap();
    connsMap.insert(tok, sock);
    counter = counter + 1;

    
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            let tok = event.token();
            if event.kind().is_readable()  {
                if tok == SERVER {
                    println!("Accepted a socket");
                    let mut client_socket = match server.accept() {
                        Err(e) => {
                            println!("Accept error: {}", e);
                            return;
                        },
                    
                        Ok((sock, _)) => sock
                    };
                    
                    let tok = Token(counter);
                    poll.register(&client_socket, 
                                tok, 
                                Ready::writable(), 
                                PollOpt::edge()/* | PollOpt::oneshot()*/).unwrap();
                    connsMap.insert(tok, client_socket);
                    counter = counter + 1;

                } else {
                    println!("Enter reading");
                    let mut socket = connsMap.get(&tok).unwrap();

                    let mut buf = [0u8; 20];

                    let len = match socket.read(&mut buf) {
                            Ok(bytesRead) => bytesRead,
                            Err(err) => {
                                if let WouldBlock = err.kind() {
                                    println!("Would block, so return");
                                    0
                                } else {
                                    println!("Error reading socket: {}", err);
                                    0xFFFFFFFF
                                }
                            }
                        };
                    println!("Reading socket: {}", str::from_utf8(&buf).unwrap());
                    
                    /*
                    poll.register(socket, 
                                    tok, 
                                    Ready::readable(), 
                                    PollOpt::edge() /*| PollOpt::oneshot()*/).unwrap();
                    */

                }
            } else {
                println!("Enter writing");
                let mut w_sock = match connsMap.get(&tok) {
                    Some(sock) => sock,
                    None => {
                        println!("Socket for write not found");
                        return;
                    }
                };
                let len = match w_sock.write(b"some bytes") {
                            Ok(bytesRead) => bytesRead,
                            Err(err) => {
                                if let WouldBlock = err.kind() {
                                    println!("Would block, so return");
                                    0
                                } else {
                                    println!("Error writing socket: {}", err);
                                    0xFFFFFFFF
                                }
                            }
                        };

                /*
                poll.register(w_sock, 
                                tok, 
                                Ready::writable(), 
                                PollOpt::edge()/* | PollOpt::oneshot()*/).unwrap()
                */
            }
        }
    }
}
*/