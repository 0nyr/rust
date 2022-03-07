use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use ansi_term::{Color};



fn get_user_message() -> String {
    println!();
    println!("{}", Color::Blue.paint(
        "Enter a message to send to the server:"
    ));
    
    let mut user_message = String::new();
    std::io::stdin().read_line(&mut user_message).unwrap();

    // the message contains a /n at the end, so we remove it
    user_message.trim_end().to_string()
}

fn main() {
    match TcpStream::connect("localhost:3333") {
        // case 1: connection succeeded
        Ok(mut stream) => {
            println!("{}", Color::Blue.paint(
                "Successfully connected to server in port 3333"
            ));

            // get the message from the user
            let mut message;
            //let msg = b"Hello!"; // this is a byte array

            // NOTE: Rust has no do-while
            loop {
                message = get_user_message();
                //std::io::stdin().read_line(&mut message).unwrap();

                // exit cases
                if message == "exit" || message == "quit" {
                    print!("{}", Color::Yellow.paint(
                        "Exiting..."
                    ));
                    break;
                }

                // send the message to the server
                println!("{}", Color::Yellow.paint(
                    format!(
                        "Sent following message: {} [nb of chars: {}]", 
                        message, message.len()
                    )
                ));
                stream.write(message.as_bytes()).unwrap();

                // read the message from the server
                let mut data = [0 as u8; 50];
                match stream.read(&mut data) {
                    // case 1: read succeeded
                    Ok(size) => {
                        let text = from_utf8(&data[0..size]).unwrap();
                        println!("{}", Color::Yellow.paint(
                            format!("Server replied with: {}", text)
                        ));

                        // check if the echo server replied correctly
                        if text == message {
                            println!("{}", Color::Green.paint(
                                "Reply is ok!"
                        ));
                        } else {
                            println!("{}", Color::Red.paint(
                                "Unexpected reply!"
                        ));
                        }
                    },
                    // case 2: read failed
                    Err(e) => {
                        println!("{}", Color::Red.paint(
                            format!("Failed to receive data: {}", e)
                        ));
                    }
                }
            }
        },

        // case 2: connection failed
        Err(e) => {
            println!("{}", Color::Red.paint(
                format!("Failed to connect: {}", e)
            ));
        }
    }
    println!("{}", Color::Yellow.paint(
        "Terminated."
    ));
}