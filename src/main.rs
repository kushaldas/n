use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, ToSocketAddrs};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
extern crate clap;
use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("To learn programming once agian")
        .version("0.1.0")
        .author("Kushal Das <mail@kushaldas.in>")
        .about("Helps me to learn")
        .arg(
            Arg::with_name("listen")
                .short("l")
                .long("listen")
                .value_name("LISTEN")
                .help("Listens to the given port")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("domain")
                .help("Domain name or IP address to connect to.")
                .index(1)
                .requires("port"),
        )
        .arg(
            Arg::with_name("port")
                .help("The port number to connect to.")
                .requires("domain")
                .index(2),
        )
        .get_matches();
    // let us see the arugments
    let listen = matches.value_of("listen").unwrap_or("0");

    // buffer
    let mut data = [0_u8; 1024];
    // If we have to listen
    if listen != "0" {
        // Let us listen to the given port
        //
        let ip = Ipv4Addr::new(0, 0, 0, 0);
        let socket = SocketAddrV4::new(ip, listen.parse::<u16>().unwrap());
        let listener = TcpListener::bind(socket).await?;
        let (mut stream, _addr) = listener.accept().await?;

        // We need stdout as we may print direct BINARY data
        let mut out = std::io::stdout();
        loop {
            let bytes_read = stream.read(&mut data).await?;
            if bytes_read == 0 {
                out.flush()?;
                break;
            }
            out.write_all(&data[..bytes_read])?;
            out.flush()?;
        }
        // All done, now return
        return Ok(());
    }
    // We will try to connect to the given domain/ip.
    let domain = match matches.value_of("domain") {
        Some(name) => name,
        None => {
            eprintln!("Provide a domain name and port number to connect to.");
            return Ok(());
        }
    };

    let port = match matches.value_of("port") {
        Some(name) => name,
        None => {
            eprintln!("Provide a port number to connect to.");
            return Ok(());
        }
    };

    // We need this to find the correct IP address
    let connection_string = format!("{}:{}", domain, port);

    // TODO: Select v4 or v6 based on user input
    let mut ip_addrs = match connection_string.to_socket_addrs() {
        Ok(ips) => ips,
        Err(_) => {
            eprintln!("Error parsing domain/IP:port combination.");
            return Ok(());
        }
    };

    // The ip address to connect to
    let _socket = ip_addrs.next().unwrap();
    dbg!(_socket);

    let mut stream = TcpStream::connect(_socket).await?;

    loop {
        match io::stdin().read(&mut data) {
            Ok(bytes_read) => {
                stream.write_all(&data[..bytes_read]).await?;
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}
