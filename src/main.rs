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

    // If we have to listen
    if listen != "0" {
        // Let us listen to the given port
        //
        let ip = Ipv4Addr::new(0, 0, 0, 0);
        let socket = SocketAddrV4::new(ip, listen.parse::<u16>().unwrap());
        let listener = TcpListener::bind(socket)?;
        let (mut stream, _addr) = listener.accept()?;

        // buffer
        let mut data = [0_u8; 1024];
        // We need stdout as we may print direct BINARY data
        let mut out = std::io::stdout();
        loop {
            let bytes_read = stream.read(&mut data)?;
            if bytes_read == 0 {
                break;
            }
            out.write_all(&data[..bytes_read])?;
            out.flush()?;
        }
        // All done, now return
        return Ok(());

    }
    Ok(())
}
