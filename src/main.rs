#![feature(slice_patterns)]
extern crate serde_json;
extern crate rnet;

use rnet::Netmessage;

fn main() {
    use std::env::args;
    use std::io::stdin;
    use std::io::{BufRead, BufReader};
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::sync::mpsc::{channel, TryRecvError};
    use std::thread;
    let mut x = 0;
    let mut y = 0;
    let mut map = [[false; 128]; 128];
    let bindaddress = args()
        .nth(1)
        .unwrap_or_else(|| panic!("Error: Pass an address in the format \"ip:port\" to bind to."));

    let mut stream = TcpStream::connect::<&str>(&bindaddress).unwrap();
    stream.write(&[42, 72, 69, 76, 76, 79, 42]).unwrap();

    let mut istream = stream.try_clone().unwrap();

    // Spawn a thread to receive Netmessage objects and send them back to the main thread.
    let (msg_sender, msg_receiver) = channel();
    thread::spawn(move || {
        loop {
            let mut header = [0u8; 6];
            let mut body = [0u8; 256];

            istream.read_exact(&mut header).unwrap();
            let body = &mut body[0..header[5] as usize + 1];
            istream.read_exact(body).unwrap();

            if let Ok(m) = serde_json::from_slice(body) {
                msg_sender.send(m).unwrap();
            } else {
                println!("Invalid message.");
            }
        }
    });

    // Spawn a thread to get lines from the stdin and send them back to the main thread.
    let (input_sender, input_receiver) = channel();
    thread::spawn(move || {
        for line in BufReader::new(stdin()).lines() {
            match line {
                Ok(line) => input_sender.send(line).unwrap(),
                Err(e) => panic!("Unable to read line: {}", e),
            }
        }
    });

    loop {
        // Handle network messages.
        match msg_receiver.try_recv() {
            Ok(m) => {
                // Match the Netmessage type.
                match m {
                    Netmessage::ReqName => {
                        serde_json::to_writer(&mut stream, &Netmessage::NameDebugJoe).unwrap();
                    }
                    Netmessage::Heartbeat => {}
                    Netmessage::ReqNetstats => {}
                    Netmessage::Movement(m) => {
                        println!("{:?}", m);
                    }
                    Netmessage::DebugJoeOC(l ,r,tl, tr) => {
                        println!("PID speed control: Left: {:?}, Right: {:?}", l ,r);
                        println!("Encoder ticks:  Left: {:?}, Right: {:?}", tl, tr);
                    }
                    Netmessage::DebugJoeDistance(d) => {
                        println!("Moved forward {:?} cm", d);
                    }
                    //Netmessage::ReqMovement => {}
                    _ => println!("Unhandled message: {:?}", m),
                }
            }
            Err(TryRecvError::Disconnected) => panic!("Connection lost."),
            Err(TryRecvError::Empty) => {}
        }

        // Handle input from terminal.
        match input_receiver.try_recv() {
            Ok(line) => {
                let words = line.split(' ').collect::<Vec<_>>();
                match words.as_slice() {
                    &["sensors", dis, lp, rp] => {
                        serde_json::to_writer(&mut stream, &Netmessage::DebugJoeUltra(
                                dis.parse().unwrap(),
                                lp.parse().unwrap(),
                                rp.parse().unwrap()
                            )).unwrap();
                    }
                        _ => println!("Usage: sensors dis lp rp"),
                }
                /*if words.len() != 0 {
                    // Match the String.
                    match words[0] {
                        "debugmove" =>{
                            serde_json::to_writer(&mut stream, &Netmessage::DebugJoeTread()).unwrap();
                        }
                        "sensors" =>{

                        }
                        _ => {
                        
                        },
                    }
                }*/
            }
            Err(TryRecvError::Disconnected) => panic!("Input lost."),
            Err(TryRecvError::Empty) => {}
        }
    }
}
