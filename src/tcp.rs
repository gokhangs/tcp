use etherparse;
use std::io;

pub enum State {
	Closed,
	Listen,
	//SynRcvd,
	//Estab,
}

//#[derive(Debug)]
pub struct Connection {
	state: State 

}

pub struct SendSequence {
	// send unacknowledge
	una: usize,
	// send next
	nxt: usize,
	// send window
	wnd: usize,
	// send urgent pointer
	up: bool,
	// seqment sequence number used for last window update
	wl1: usize,
	// seqment acknowledgement number used for last window update
	wl2: usize,
	// inital send sequence number
	iss: usize,
}


pub struct ReceiveSequence {
	// send next
	nxt: usize,
	// send window
	wnd: usize,
	// send urgent pointer
	up: bool,
	// inital receive sequence number
	irs: usize,
}

impl Default for Connection {
	fn default() -> Self {
		//State::Closed
		Connection {
			state: State::Listen
		}
	}
}

impl Connection {
    pub fn on_packet<'a>(
    	&mut self,
    	nic: &mut tun_tap::Iface,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
    	let mut buff = [0u8; 1500];
    	match self.state {
    		State::Closed => {
    			return Ok(0);
    		}
    		State::Listen => {
    			if !tcp_header.syn() {
    				//only expected SYN packet
    				return Ok(0);
    			}
    			//establish connection, reply with Syn Ack
    			let mut syn_ack = etherparse::TcpHeader::new(
    					tcp_header.destination_port(),
    					tcp_header.source_port(),
    					unimplemented!(),
    					unimplemented!(),
    				);
    			syn_ack.syn = true;
    			syn_ack.ack = true;

    			let mut ip = etherparse::Ipv4Header::new(
    					syn_ack.header_len(),
    					64,
    					etherparse::IpNumber::Tcp as u8,
    					[
    						ip_header.destination()[0],
    						ip_header.destination()[1],
    						ip_header.destination()[2],
    						ip_header.destination()[3],
    					],
    					    					[
    						ip_header.source()[0],
    						ip_header.source()[1],
    						ip_header.source()[2],
    						ip_header.source()[3],
    					],
    				);
    			//use entrie buffer to write, and find how much spaces haven't been used yet
				let mut unwritten = {
					let mut unwritten = &mut buff[..];
	    			ip.write(&mut unwritten);
	    			syn_ack.write(&mut unwritten);
	    			//used to calculate how much space is remaining in the buffer
	    			unwritten.len()
				};
				nic.send(&buff[..unwritten])

    		}

    	}
        // eprintln!(
        //     "{}: {} -> {}:{}  {}b of TCP",
        //     ip_header.source_addr(),
        //     tcp_header.source_port(),
        //     ip_header.destination_addr(),
        //     tcp_header.destination_port(),
        //     data.len(),
        // );
    }
}
