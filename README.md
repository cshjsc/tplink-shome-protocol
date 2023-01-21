# TP Link home protocol for rust

Simple library to easily communicate with a tp link smart device.
Made possible by this [article](https://www.softscheck.com/en/reverse-engineering-tp-link-hs110/#TP-Link%20Smart%20Home%20Protocol).

## Usage

```rust
use std::io;
use std::net::TcpStream;

use tplink_shome_protocol::{receive_message, send_message};

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("192.168.1.1:9999")?;
    let raw = r#"{"system":{"get_sysinfo":{}}}"#;
    send_message(&stream, raw)?;
    let message = receive_message(&stream)?;
    ...
}
```
