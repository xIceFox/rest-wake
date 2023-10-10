use std::iter;
use std::net::{SocketAddr, UdpSocket};
use crate::network::mac_address::MacAddress6;

const MAGIC_HEADER: [u8; 6] = [0xFF; 6];
const NUMBER_OF_MAC_REPETITIONS: usize = 16;

pub fn send_wol(mac : MacAddress6) -> Result<(), String> {
    let mut mac_vec : Vec<MacAddress6> = Vec::new();
    mac_vec.push(mac);
    send_wol_packets(mac_vec)?;
    Ok(())
}

pub fn send_wol_packets(mac_addresses : Vec<MacAddress6>) -> Result<(), String> {
    let socket = UdpSocket::bind(SocketAddr::from(([0,0,0,0],0))).map_err(|_err| String::from("Error binding to network interface"))?;
    socket.set_broadcast(true).map_err(|_err| String::from("Error setting SO_BROADCAST"))?;

    for mac in mac_addresses {
        socket.send_to(&craft_magic_packet(mac), SocketAddr::from(([255,255,255,255],9))).map_err(|_err| String::from("Could not send packet"))?;
    }

    Ok(())
}

pub fn craft_magic_packet(mac : MacAddress6) -> Vec<u8> {
    let mut packet : Vec<u8> = Vec::new();

    packet.extend(MAGIC_HEADER);

    packet.extend(iter::repeat(mac)
        .take(NUMBER_OF_MAC_REPETITIONS)
        .flatten()
        .collect::<Vec<u8>>());

    return packet;
}