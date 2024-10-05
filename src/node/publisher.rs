use super::message::FLSMsg;

use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

pub struct Publisher<T : FLSMsg>
{
    name : String,
    sender : Sender<String>,
    value : Option<T>
}

impl<T : FLSMsg> Publisher<T> {
    pub fn new(node_name : String)->Publisher<T>
    {
        let (t, r) = std::sync::mpsc::channel::<String>();

        std::thread::spawn(move ||{
            let sock = std::net::UdpSocket::bind("127.0.0.1:").unwrap();

            loop {
                let v = r.recv().unwrap();

                match sock.send_to(v.as_bytes(), "asd") {
                    Ok(_s)=>{

                    }
                    Err(_e)=>{

                    }
                }
            }
        });

        Self {name : node_name, sender: t, value : None}
    }

    pub fn publish(&self, value : T)
    {
        let _ = self.sender.send(value.create_packet());
    }
}