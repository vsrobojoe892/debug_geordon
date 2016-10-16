#![feature(proc_macro)]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Point {
    /// This is in meters.
    pub x: f64,
    /// This is in meters.
    pub y: f64,
    /// This is a variance in meter squared.
    pub v: f64,
    /// This is the angle in radians.
    pub angle: f64,
    /// This is the angle's variance in radians^2.
    pub av: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Netmessage {
    ReqName,
    NameJosh,
    NameGeordon,
    NameZach,
    NameJoe,
    NameDebugJosh,
    NameDebugGeordon,
    NameDebugZach,
    NameDebugJoe,
    Netstats {
        #[serde(rename = "myName")]
        my_name: String,
        #[serde(rename = "numGoodMessagesRecved")]
        num_good_messages_recved: u32,
        #[serde(rename = "numCommErrors")]
        num_comm_errors: u32,
        #[serde(rename = "numJSONRequestsRecved")]
        num_json_requests_recved: u32,
        #[serde(rename = "numJSONResponsesRecved")]
        num_json_responses_recved: u32,
        #[serde(rename = "numJSONRequestsSent")]
        num_json_requests_sent: u32,
        #[serde(rename = "numJSONResponsesSent")]
        num_json_responses_sent: u32,
    },
    Heartbeat,
    ReqNetstats,
    /// Joe
    ReqMovement,
    /// Geordon
    Movement(Point),
    /// Geordon
    JoeReqPoints,
    /// Joe
    JF(u32),
    /// Joe
    JE(u32),
    /// Geordon
    JoshReqPoints,
    /// Josh
    CF(u32),
    /// Josh
    CE(u32),
    /// Josh
    CT(u32),
    /// Geordon
    ReqStopped,
    /// Josh
    Stopped(bool),
    /// Josh
    ReqInPosition,
    /// Zach
    InPosition(bool),
    /// Zach
    ReqEdgeDetect,
    /// Josh
    EdgeDetect(bool),
    /// Zach
    ReqEdgeDropped,
    /// Josh
    EdgeDropped(bool),
    /// Zach
    ReqDistance,
    /// Josh; Value is in meters.
    Distance(f64),
    /// Zach
    ReqGrabbed,
    /// Josh
    Grabbed(bool),
    /// Zach
    ReqDropped,
    /// Josh
    Dropped(bool),
}

fn main() {
    use std::env::args;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    let bindaddress = args()
        .nth(1)
        .unwrap_or_else(|| panic!("Error: Pass an address in the format \"ip:port\" to bind to."));
    let mut stream = TcpStream::connect::<&str>(&bindaddress).unwrap();

    stream.write(&[42, 72, 69, 76, 76, 79, 42]).unwrap();

    let mut header = [0u8; 6];
    let mut body = [0u8; 256];

    loop {
        stream.read_exact(&mut header).unwrap();
        let body = &mut body[0..header[5] as usize + 1];
        stream.read_exact(body).unwrap();

        if let Ok(m) = serde_json::from_slice(body) {
            match m {
                Netmessage::ReqName => {
                    serde_json::to_writer(&mut stream, &Netmessage::NameDebugGeordon).unwrap();
                }

                Netmessage::Heartbeat => {}
                Netmessage::ReqNetstats => {

                }
                _ => println!("Unhandled message: {:?}", m),
            }
        } else {
            println!("Invalid message.");
        }
    }
}
