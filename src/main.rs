use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use tplink_shome_protocol::transport::{decrypt, encrypt};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("192.168.1.110:9999")?;
    let raw = r#"{"system":{"get_sysinfo":{}}}"#;
    //let raw = r#"{"smartlife.iot.smartbulb.lightingservice": {"transition_light_state": {"on_off": 0, "ignore_default": 1}}}"#;
    let raw = r#"{"smartlife.iot.smartbulb.lightingservice": {"transition_light_state": {"on_off": 1, "ignore_default": 0}}}"#;
    //let raw = r#"{'smartlife.iot.smartbulb.lightingservice': {'get_light_state': None}}"#;
    let encrypted = encrypt(raw.as_bytes());
    let b32len: u32 = encrypted.len() as u32;
    let lenbytes = b32len.to_be_bytes();
    stream.write_all(&lenbytes)?;
    stream.write_all(&encrypted)?;
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
    Ok(())
}
