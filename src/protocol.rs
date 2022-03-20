use crate::csvparse::{parse_csv};
use crate::prepare::{get_size, decode_str};
use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};

pub fn handle_client(mut stream: TcpStream) {  
    let mut b64_str = "".to_string();
    let mut calc_size: usize = 0;
    let mut is_set = false;

    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 16usize {
                calc_size = get_size(String::from_utf8(data.to_vec()).unwrap());

                if calc_size == 0usize && !is_set {
                    let code_err: [u8; 1] = [12];
                    stream.write(&code_err).unwrap();
                    stream.shutdown(Shutdown::Both).unwrap();
                    return
                }

                is_set = true;

                let code_success: [u8; 1] = [13];
                stream.write(&code_success).unwrap();
            }

            if is_set && calc_size > 0 {
                b64_str.push_str(String::from_utf8(data.to_vec()).unwrap().as_str());
                calc_size -= size;
            }

            if is_set && calc_size == 0 {
                let decoded = decode_str(b64_str.clone().as_str());

                if decoded == "123".to_string() {
                    let resp = "Error deocding string".as_bytes();
                    stream.write(&resp).unwrap();
                    stream.shutdown(Shutdown::Both).unwrap();
                    return
                } else {
                    let resp = parse_csv(decoded.clone());

                    stream.write(&resp.as_bytes()).unwrap();
                    stream.shutdown(Shutdown::Both).unwrap();
                    return

                }
            }

            let resp = "Unknown data".as_bytes();

            stream.write(&resp).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}