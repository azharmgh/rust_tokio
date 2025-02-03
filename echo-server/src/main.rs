use std::net::TcpListener;
use std::io::Write;
use std::io::Read;
use std::env::args;


const SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main() {

    let delay = args().nth(1).unwrap_or("0".to_string()).parse::<u64>().unwrap();
    if delay > 0 {
        println!("Delaying for {} seconds", delay);
        
    }
    println!("Starting echo server at {}", SERVER_ADDRESS);

    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        std::thread::spawn(move || {
            handle_connection(stream, delay);
        });
    }
}


fn handle_connection(mut stream: std::net::TcpStream, delay: u64) {
    let mut buffer = [0; 1024];

    let len = stream.read(&mut buffer).unwrap();

    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("Request: {}", String::from_utf8_lossy(&buffer));
    std::thread::sleep(std::time::Duration::from_secs(delay));
    let _ = stream.write_all(message.as_bytes());
    //stream.flush().unwrap();
}
   
