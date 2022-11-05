# tcp
Attempt to write TCP in Rust, for fun.


Methodology: 

tcp interacts with the Kernel via the TUN device (https://www.kernel.org/doc/Documentation/networking/tuntap.txt) which is essentially working as virtual network interface for us, and treated by Kernel as its own network card. Enables us to emulated network inside User Space:

    Any send operation done by Kernel will be received from the tcp running on the User Space. 

    Any write by tcp will go thorugh the TUN and appear to Kernel as it is coming from external network.


##Setup
Works with Unix based systems, which provices configurable capabilities (see: man capabilities). Latest OSX releases are moving away from kernel extensions and they are not recommneded.  

Execution binary should be provided with CAP_NET_ADMIN capability, which enables tcp to run various network related operations. Then, ip address of the new tun must be set with "ip addr set".
