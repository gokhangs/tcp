use etherparse;
use std::collections::hash_map::{Entry, HashMap};
use std::io;
use std::net::Ipv4Addr;
use tcp::tcp::Connection;
use tun_tap;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    source: (Ipv4Addr, u16),
    destination: (Ipv4Addr, u16),
}

fn main() -> std::io::Result<()> {
    //All TCP connections
    let mut connections: HashMap<Quad, Connection> = Default::default();
    let mut nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to CI");
    //set the buffer to receive
    let mut buf = [0u8; 1504];
    loop {
        let nic_bytes = nic.recv(&mut buf)?;
        let eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            //not ipv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nic_bytes]) {
            Ok(ip_header) => {
                let src = ip_header.source_addr();
                let dst = ip_header.destination_addr();
                if ip_header.protocol() != 0x06 {
                    //not a tcp packet
                    continue;
                }
                match etherparse::TcpHeaderSlice::from_slice(
                    &buf[4 + ip_header.slice().len()..nic_bytes],
                ) {
                    Ok(tcp_header) => {
                        //place that includes all the header info, rest will be TCP packet
                        let datai = 4 + ip_header.slice().len() + tcp_header.slice().len();

                        match connections.entry(Quad {
                            source: (src, tcp_header.source_port()),
                            destination: (dst, tcp_header.destination_port()),
                        }) {
                            //If this quad exists in the connections map, use that connection to handle incoming packet
                            Entry::Occupied(mut c) => {
                                c.get_mut().on_packet();
                            }
                            //If this quad is not in our connections map, initiate a new connection via accept method
                            Entry::Vacant(e) => {
                                if let Some(c) = Connection::accept(
                                    &mut nic,
                                    ip_header,
                                    tcp_header,
                                    &buf[datai..nic_bytes],
                                )? {
                                    e.insert(c);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Ignoring packet {:}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Ignoring packet {:}", e);
            }
        }
    }

    Ok(())
}
