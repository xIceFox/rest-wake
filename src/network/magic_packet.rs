use std::io::Error;
use std::iter;
use std::net::{SocketAddr, UdpSocket};

const MAGIC_HEADER: [u8; 6] = [0xFF; 6];

//send wol

//send multiple wol

pub fn send_wol(mac : [u8; 6]) -> Result<(), Error> {
    let socket = UdpSocket::bind(SocketAddr::from(([0,0,0,0],0)))?;
    let mut packet : Vec<u8> = Vec::new();

    packet.extend(MAGIC_HEADER);

    packet.extend(iter::repeat(mac)
        .take(16)
        .flatten()
        .collect::<Vec<u8>>());

    socket.set_broadcast(true)?;
    socket.send_to(&packet, SocketAddr::from(([255,255,255,255],9)))?;

    Ok(())
}

