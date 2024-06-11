use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

   for stream in listener.incoming() {
       let stream = stream.unwrap();
       handle_connection(stream);
   }

}

// This function handles a TCP connection
fn handle_connection(mut stream: TcpStream){
    // Create a buffer of 512 bytes
    let mut buffer = [0; 512];
    // Read the data from the TcpStream into the buffer
    stream.read(&mut buffer).unwrap();

    // Read the contents of the "hello.html" file into a string
    let contents = fs::read_to_string("hello.html").unwrap();
    // Create an HTTP response with the contents of the "hello.html" file
    let response = format!(
        "HTTP/1.1 200 OK\r\n\r\n{}",
        contents,
    );
    // Write the HTTP response to the TcpStream
    stream.write(response.as_bytes()).unwrap();
    // Flush the TcpStream
    stream.flush().unwrap();
}
