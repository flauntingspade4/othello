use std::net::{TcpListener, TcpStream};

use std::io::{Read, Result, Write};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4651")?;

    let mut owner = 0;

    let mut streams = Vec::with_capacity(2);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        streams.push(stream);

        owner += 1;

        if owner > 1 {
            match handle_all(&mut streams) {
                Ok(_) => break,
                Err(e) => println!("Problem with server {}", e),
            }
        }
    }

    Ok(())
}

fn handle_all(streams: &mut Vec<TcpStream>) -> Result<()> {
    for (index, stream) in streams.iter_mut().enumerate() {
        stream.write_all(&[index as u8])?;
    }

    let mut turn = 0;

    loop {
        let mut buf = [0; 2];
        streams[turn].read_exact(&mut buf)?;
        if buf[0] == 0 {
            return Ok(());
        }
        for (i, stream) in streams.iter_mut().enumerate() {
            if i != turn {
                stream.write_all(&buf)?;
            }
        }

        turn += 1;
        turn %= 2;
    }
}
