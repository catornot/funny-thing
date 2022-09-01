use std::net::TcpListener; 
use std::net::TcpStream;
use std::io::prelude::*;
use uwuifier::uwuify_str_sse;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args[1] == "cake" { 

        let layers:i32 = match args[2].parse() {
            Err( _ ) => 1,
            Ok( result ) => result
        };
        // cake_generator( layers );
        println!( "{}", layers )
    }


    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    for stream in listener.incoming(){
        let stream: TcpStream = stream.unwrap();
        println!("connection does thing");
        
        handle_connection( stream );
    }
    println!("hewwo world!"); // we have an owofier for a reason

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).expect("couldn't read stream");

    let mesasge = String::from_utf8_lossy( &buffer );

    let uwu_message = uwuify_str_sse( &mesasge );

    stream.write( uwu_message.as_bytes() ).unwrap();
    stream.flush().unwrap();
}

// I trust you <3 // mistake <3
/* not done but cant finish rn
fn cake_generator(layers: i32) -> String { // let this show little rust i actually know 
    struct cake_parts { // shut up camel case
        cake_top1layer: String,
        cake_bottom1layer: String,
        cake_top_rcrnr: String,
        cake_top_lcrnr: String,
        cake_line_top: String,
        cake_line_bottom: String,
    }
    let cake_parts = cake_parts {
        cake_top1layer: String::from("/▔▔\"")
    }
}
*/