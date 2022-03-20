mod prepare;
mod csvparse;
mod protocol;

use std::thread;
use std::net::{TcpListener};


#[test]
fn test_csv() {
    let csvstr = "42, 160, 28, 10, 5, 3,  60, 0.28,  3167
    175, 180, 18,  8, 4, 1,  12, 0.43,  4033
    129, 132, 13,  6, 3, 1,  41, 0.33,  1471
    138, 140, 17,  7, 3, 1,  22, 0.46,  3204
    232, 240, 25,  8, 4, 3,   5, 2.05,  3613
    135, 140, 18,  7, 4, 3,   9, 0.57,  3028
    150, 160, 20,  8, 4, 3,  18, 4.00,  3131
    207, 225, 22,  8, 4, 2,  16, 2.22,  5158".to_string();
    let parsed = csvparse::parse_csv(csvstr);
    println!("{}", parsed);
}


#[test]
fn test_size() {
    println!("{}", prepare::get_size("SET 0232".to_string()));
}


fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    protocol::handle_client(stream)
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