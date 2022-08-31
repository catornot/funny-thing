use std::net::TcpListener; 
use std::net::TcpStream;
use std::io::prelude::*;
use uwuifier::uwuify_str_sse;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    for stream in listener.incoming(){
        let stream: TcpStream = stream.unwrap();
        println!("connection does thing");
        
        handle_connection( stream );
    }
    println!("hewwo world!");

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).expect("couldn't read stream");

    let mesasge = String::from_utf8_lossy( &buffer );

    let uwu_message = uwuify_str_sse( &mesasge );

    stream.write( uwu_message.as_bytes() ).unwrap();
    stream.flush().unwrap();
}

// I trust you <3