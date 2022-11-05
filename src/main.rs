use std::io;
use tun_tap;

fn main() -> std::io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to CI");
    //set the buffer to receive
    let mut buf = [0u8; 1504];
    loop {
        let nic_bytes = nic.recv(&mut buf)?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        eprintln!(
            "read {} bytes *(flags: {:x}, proto: {:x}),  data: {:x?}",
            nic_bytes - 4,
            flags,
            proto,
            &buf[4..nic_bytes]
        );
    }

    Ok(())
}
