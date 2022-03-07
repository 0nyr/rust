use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use ansi_term::{Color};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("{}", Color::Red.paint(
                format!(
                    "An error occurred, terminating connection with {}", 
                    stream.peer_addr().unwrap()
                )
            ));
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them
    println!("{}", Color::Yellow.paint(
        "Server listening on port 3333"
    ));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("{}", Color::Green.paint(
                    format!(
                        "New connection: {}", 
                        stream.peer_addr().unwrap()
                    )
                ));
                // spawn a new thread for each connection
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                /* connection failed */
                println!("{}", Color::Red.paint(
                    format!("Error: {}", e)
                ));
            }
        }
    }
    // close the socket server
    drop(listener);
}