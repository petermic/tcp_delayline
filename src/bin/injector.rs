use std::net::{TcpStream,SocketAddr};
use std::io::Write;

fn main() {
    let mut args = std::env::args().skip(1);
    let addr = args.next();
    let data = args.next();
    if addr.is_some() && data.is_some() {
        let address = addr.unwrap();
        let datagram = data.unwrap();
        let mut stream = TcpStream::connect(address.parse::<SocketAddr>().expect("injector parse failed"))
            .expect("injector connection failed");
        stream.set_nodelay(true).unwrap();
        println!("connected");
        let msg = datagram+"\n";
        let bytes = msg.as_bytes();
        stream.write(bytes).expect("injection failed");
        println!("sent");
    }
    else {
        println!("Usage: ./injector [address] [data] [source address (opt.)]");
    }
}
