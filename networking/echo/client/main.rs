use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn get_user_message() -> String {
    let mut user_message = String::new();
    println!("Enter a message to send to the server:");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_message).unwrap();
    user_message
}

fn main() {
    match TcpStream::connect("localhost:3333") {
        // case 1: connection succeeded
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            // get the message from the user
            let mut message;
            //let msg = b"Hello!"; // this is a byte array

            // NOTE: Rust has no do-while
            loop {
                message = get_user_message();
                //std::io::stdin().read_line(&mut message).unwrap();

                // exit cases
                if message == "exit" || message == "quit" {
                    print!("Exiting...");
                    break;
                }

                // send the message to the server
                stream.write(message.as_bytes()).unwrap();

                // read the message from the server
                let mut data = [0 as u8; 50];
                match stream.read(&mut data) {
                    // case 1: read succeeded
                    Ok(size) => {
                        let text = from_utf8(&data[0..size]).unwrap();
                        println!("Server replied with: {}", text);
                        if &data == message.as_bytes() {
                            println!("Reply is ok!");
                        } else {
                            println!("Unexpected reply!");
                        }
                    },
                    // case 2: read failed
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        },

        // case 2: connection failed
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}