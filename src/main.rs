use std::io;
use tun_tap;
use etherparse;

fn main() -> std::io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to CI");
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
            Ok(p) => {
               let src = p.source_addr();
               let dst = p.destination_addr();
               let proto = p.protocol();
               if proto != 0x06 {
                  //not a tcp packet
                  continue;
               }

               match etherparse::TcpHeaderSlice::from_slice(&buf[4+p.slice().len()..nic_bytes]) {
                  Ok(p) => {
                     eprintln!(
                        "{} -> {} {}b of TCP to port {}",
                        src,
                        dst,
                        p.slice().len(),
                        p.destination_port()
                     );
                  },
                  Err(e) => {
                     eprintln!("Ignoring packet {:}", e);
                  }
               }
            },
            Err(e) => {
               eprintln!("Ignoring packet {:}", e);
            }
        }


    }

    Ok(())
}
