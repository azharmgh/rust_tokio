
 use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

const ECHO_SERVER: &str = "localhost:1234";

#[tokio::main]
async fn main(){

    println!("Connecting to echo server at {}", ECHO_SERVER);
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER).await{
    
        println!("Connected to echo server at {}", ECHO_SERVER);
        let message = "Hello from client";
        let _ = stream.write(message.as_bytes()).await;
        let _ = stream.flush().await;
        let mut response = [0; 128];
        let len = stream.read(&mut response).await.unwrap();
        let reply = std::str::from_utf8(&response[..len]).unwrap();
        println!("Response: {}", reply);
    }
        // .map(|mut stream| {
        //     println!("Connected to echo server at {}", ECHO_SERVER);
        //      loop {
        //         let mut input = String::new();
        //         std::io::stdin().read_line(&mut input).unwrap();
        //         stream.write(input.as_bytes()).unwrap();
        //         let mut response = [0; 128];
        //         stream.read(&mut response).unwrap();
        //         println!("Response: {} from {}:{}", String::from_utf8_lossy(&response), stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
        //      }
        // })
        // .unwrap_or_else(|e| {
        //     eprintln!("Failed to connect to echo server at {}: {}", ECHO_SERVER, e);
        // }); 
}