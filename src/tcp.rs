use etherparse;


pub struct State {
}

impl Default for State {
	fn default() -> Self {
		State {}
	}
}

impl State {
    pub fn on_packet(
    	&mut self,
        ip_header: etherparse::Ipv4HeaderSlice,
        tcp_header: etherparse::TcpHeaderSlice,
        data: &[u8],
    ) {
        eprintln!(
            "{}: {} -> {}:{}  {}b of TCP",
            ip_header.source_addr(),
            tcp_header.source_port(),
            ip_header.destination_addr(),
            tcp_header.destination_port(),
            data.len(),
        );
    }
}
