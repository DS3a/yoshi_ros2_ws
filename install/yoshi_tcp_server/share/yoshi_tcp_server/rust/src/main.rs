use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use serde_json;
use yoshi_msgs::yoshi_msgs;


fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 500]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            let mut str_data = std::str::from_utf8(&data).unwrap();
            str_data = str_data.trim_matches(char::from(0));
            let msg_from_esp: yoshi_msgs::MsgFromEsp = serde_json::from_str(str_data).unwrap();
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            println!("got data: {:?}", msg_from_esp);
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("11.42.0.2:3000").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
