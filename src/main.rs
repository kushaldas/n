use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
extern crate clap;
use clap::{App, Arg};

fn main() -> Result<(), Error> {
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
        .get_matches();
    // let us see the arugments
    let listen = matches.value_of("listen").unwrap_or("0");

    if listen != "0" {
        // Let us listen to the given port
        //
        let ip = Ipv4Addr::new(0, 0, 0, 0);
        let socket = SocketAddrV4::new(ip, listen.parse::<u16>().unwrap());
        let listener = TcpListener::bind(socket)?;
        let (mut stream, _addr) = listener.accept()?;

        // buffer
        let mut data = [0 as u8; 1024];
        loop {
            match stream.read(&mut data) {
                Ok(size) => {
                    if size == 0 {
                        // Means goodbye
                        break;
                    }
                    let mut out = std::io::stdout();
                    out.write_all(&data[..size])?;
                    out.flush()?;
                }
                Err(err) => {
                    println!("{:?}", err);
                    break;
                }
            }
        }
    }
    Ok(())
}
