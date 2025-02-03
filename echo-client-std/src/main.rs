 use std::io::prelude::*;
 use std::net::TcpStream;

const ECHO_SERVER: &str = "localhost:1234";


fn main(){

    println!("Connecting to echo server at {}", ECHO_SERVER);
    TcpStream::connect(ECHO_SERVER)
        .map(|mut stream| {
            println!("Connected to echo server at {}", ECHO_SERVER);
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                stream.write(input.as_bytes()).unwrap();
                let mut response = [0; 128];
                stream.read(&mut response).unwrap();
                println!("Response: {} from {}:{}", String::from_utf8_lossy(&response), stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
            }
        })
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect to echo server at {}: {}", ECHO_SERVER, e);
        }); 
}