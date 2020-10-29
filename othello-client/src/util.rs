use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

pub fn connect(ip: Vec<u8>) -> Result<TcpStream, std::io::Error> {
    let ip = Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]);
    let socket_all = SocketAddrV4::new(ip, 4651);

    TcpStream::connect(socket_all)
}

pub fn turn_to_nums(ip: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    let mut ip_vec = Vec::new();
    let ip: Vec<&str> = ip.split('.').collect();
    for addr in ip {
        match addr.parse::<u8>() {
            Ok(t) => ip_vec.push(t),
            Err(e) => return Err(e),
        }
    }
    Ok(ip_vec)
}
