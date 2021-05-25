use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

fn main() {
  // listen localhost server port 7878 as entry point
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  // loop for any incoming request
  for stream in listener.incoming() {
    // console log for separating each request
    println!("------------- waiting new request -------------");

    // error handling on stream is None
    match stream {
      Ok(stream) => {
        thread::spawn(move || {
          // connection succeeded
          // handle_connection function with stream as arguement
          handle_connection(stream)
        });
      }
      Err(e) => {
        // connection failed
        // print out error
        println!("Error: {}", e);
      }
    }
  }
}

// function handle_connect delcaration, it will receive a TcpSteam type arguement call stream
fn handle_connection(mut stream: TcpStream) {
  // declare buffer for getting request string
  let mut buffer = [0 as u8; 1024];
  // read from the TcpStream and put them in the buffer
  stream.read(&mut buffer).unwrap();
  // The String::from_utf8_lossy function takes a &[u8] and produces a String from it.
  let response = &buffer[..];
  // store text into response_text
  let response_text = from_utf8(&buffer).unwrap();

  // console log for separating each Response
  println!("------------- Response -------------");
  // print out response
  println!("Response: {}", response_text);

  // echo response!
  stream.write(&response).unwrap();
}
