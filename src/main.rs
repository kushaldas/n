use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4, ToSocketAddrs};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
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

    // If we have to listen
    let mut stream = if listen != "0" {
        // Let us listen to the given port
        //
        let ip = Ipv4Addr::new(0, 0, 0, 0);
        let socket = SocketAddrV4::new(ip, listen.parse::<u16>().unwrap());
        let listener = TcpListener::bind(socket).await?;
        let (stream, _addr) = listener.accept().await?;
        stream

    } else {
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

        let stream = TcpStream::connect(_socket).await?;
        stream
    };

    let (mut tx, mut rx) = stream.split();

    // buffer
    let mut reader_buf = [0_u8; 1024];
    let mut writer_buf = [0_u8; 1024];

    // For input/output
    let mut out = tokio::io::stdout();
    let mut stdin = tokio::io::stdin();

    loop {
        tokio::select! {
            bytes_read = tx.read(&mut reader_buf) => {
                let actual_read = bytes_read.unwrap();
                if actual_read == 0 {
                    // Means nothing to read
                    // End of loop
                    out.flush().await?;
                    // The following break will not work
                    // Read https://github.com/tokio-rs/tokio/issues/2318 for more details
                    break;
                } else {
                    out.write_all(&reader_buf[..actual_read]).await?;
                    out.flush().await?;
                }
            }
            bytes_read = stdin.read(&mut writer_buf[..]) => {
                let actual_read = bytes_read.unwrap();
                if actual_read == 0 {
                    break;
                } else {
                    rx.write_all(&writer_buf[..actual_read]).await?;
                }
            }
        };
    }

    Ok(())
}
