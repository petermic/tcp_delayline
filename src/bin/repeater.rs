use std::net::{TcpListener,TcpStream,SocketAddr};
use std::thread;
use std::sync::mpsc;
use std::io::{BufReader,BufRead,Read,Write,stdin};
use threadpool::ThreadPool;

fn start_repeater(home_address: String, line_address: String)
{
    let (tx,rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);
    // connector thread
    thread::spawn(move || {
        println!("Set up all nodes, then press ENTER...");
        stdin().read(&mut [0]).unwrap();
        let mut stream = TcpStream::connect(line_address.parse::<SocketAddr>().expect("parse failed"))
                             .expect("TCP connection failed");
        stream.set_nodelay(true).unwrap();
        loop {
            let data = rx.recv().expect("error on MPSC channel receive");
            println!("connector thread received {}", data);
            stream.write(data.as_bytes()).expect("write failed");
            println!("wrote {}",data);
        }
    });
    let listener = TcpListener::bind(home_address.parse::<SocketAddr>().expect("parse failed"))
            .expect("Listener bind failed");
    for stream in listener.incoming()
    {
        let txc = tx.clone();
        pool.execute(move || {
            println!("incoming stream");
            let s = stream.expect("stream retrieval error");
            let mut reader = BufReader::new(&s);
            loop {
                let mut buf = String::new();
                reader.read_line(&mut buf).expect("read failed");
                if buf.len() == 0 { continue; }
                println!("read complete");
                //let sl = &buf[0..nbytes];
                //let data = sl.to_owned() as Vec<u8>;
                println!("received '{}'",buf);
//                  std::str::from_utf8(&data).expect("UTF8 parse error"));
                txc.send(buf).expect("error on MPSC channel send");
                println!("channel data sent");
            }
        });
    }
}

fn main()
{
    // perform basic input checking
    let mut args = std::env::args().skip(1);
    let home_addr = args.next();
    let line_addr = args.next();
    if home_addr.is_some() && line_addr.is_some(){
        let home_address = home_addr.unwrap();
        let line_address = line_addr.unwrap();
        start_repeater(home_address,line_address);
    }
    else {
        println!("Usage: ./repeater [home address] [line address]");
    }
}
