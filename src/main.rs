use tun_tap;
use std::io;

 fn main() -> std::io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to CI");
    //set the buffer to receive
    let mut buf = [0u8; 1504];
    let nic_bytes = nic.recv(&mut buf)?;
    eprintln!("read {} bytes {:?}", nic_bytes, &buf[..nic_bytes]);
    Ok(())
}

