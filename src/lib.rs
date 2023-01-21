use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

pub fn encrypt(data: &[u8]) -> Vec<u8> {
    let mut encryption_key = 0xAB;
    data.iter()
        .map(|e| {
            let res = e ^ encryption_key;
            encryption_key = res;
            res
        })
        .collect()
}

pub fn decrypt(encrypted: &[u8]) -> Vec<u8> {
    let mut encryption_key = 0xAB;
    encrypted
        .iter()
        .map(|e| {
            let res = e ^ encryption_key;
            encryption_key = *e;
            //println!("encryption key : {encryption_key}");
            res
        })
        .collect()
}

///
/// Blocks until it receives a message from the [TcpStream]
///
/// Example
/// ```
///use std::io;
///use std::net::TcpStream;
///
///use tplink_shome_protocol::receive_message;
///
///fn main() -> io::Result<()> {
///    let stream = TcpStream::connect("192.168.1.1:9999")?;
///    let response = receive_message(&stream)?;
///    ...
///}
/// ```
///
pub fn receive_message(mut stream: &TcpStream) -> io::Result<String> {
    let mut lenbuff = [0; 4];
    stream.read_exact(&mut lenbuff)?;
    let num = u32::from_be_bytes(lenbuff);
    let mut buffer: Vec<u8> = vec![0; num as usize];
    stream.read_exact(&mut buffer)?;
    let decrypted = decrypt(&buffer);
    let s = match std::str::from_utf8(&decrypted) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("String: {}", s);
    Ok(s.to_string())
}

///
/// Sends a message to the smart device connected via the [TcpStream]
///
/// Example
/// ```
///use std::io;
///use std::net::TcpStream;
///
/// use tplink_shome_protocol::send_message;
///
///fn main() -> io::Result<()> {
///    let stream = TcpStream::connect("192.168.1.1:9999")?;
///    let raw = r#"{"system":{"get_sysinfo":{}}}"#;
///    let response = receive_message(&stream)?;
///    ...
///}
/// ```
///
pub fn send_message(mut stream: &TcpStream, message: &str) -> io::Result<()> {
    let encrypted = encrypt(message.as_bytes());
    let b32len: u32 = encrypted.len() as u32;
    let lenbytes = b32len.to_be_bytes();

    // send first length of data
    stream.write_all(&lenbytes)?;

    stream.write_all(&encrypted)?;
    Ok(())
}
