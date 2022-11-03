# tcp
Attempt to write TCP in Rust, for fun.


Methodology: 

tcp interacts with the Kernel via the TUN device (https://www.kernel.org/doc/Documentation/networking/tuntap.txt) which is essentially working as virtual network interface for us, and treated by Kernel as its own network card. Enables us to emulated network inside User Space:

    Any send operation done by Kernel will be received from the tcp running on the User Space. 

    Any write by tcp will go thorugh the TUN and appear to Kernel as it is coming from external network.

