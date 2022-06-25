use std::{net::TcpListener, io::{Read, Write}};

const SERVER_ADDRESS: &str = "0.0.0.0:6429";

fn main() {
    let listener = TcpListener::bind(SERVER_ADDRESS).expect("could not open socket");

    if let Some(Ok(mut stream)) = listener.incoming().next() {
        let mut buffer = vec![0; 2048].into_boxed_slice();
        let size = stream.read(&mut buffer).expect("could not read from socket");

        println!("got {:?}", &buffer[..size]);
        let _ = stream.write(&buffer[..size]).expect("could not write to socket");
    }
}
