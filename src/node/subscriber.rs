use super::message::FLSMsg;
use super::utils::name_to_port;

use std::sync::mpsc::Receiver;
use std::net::UdpSocket;

pub struct Subscriber<T : FLSMsg>
{
    receiver : Receiver<String>,
    value : Option<T>
}

impl<T : FLSMsg> Subscriber<T> {
    pub fn new(topic_name : String)->Subscriber<T>
    {
        let port = name_to_port(topic_name);
        let sub_addr = format!("127.0.0.1:{}", port);

        let (t, r) = std::sync::mpsc::channel::<String>();

        std::thread::spawn(move ||{
            let sock = UdpSocket::bind(sub_addr.as_str()).unwrap();

            loop {
                let mut buf = [0_u8; 256];

                match sock.recv(&mut buf) {
                    Ok(size)=>{
                        let get_data = &buf[..size];

                        let string_data = String::from_utf8_lossy(get_data).to_string();

                        let _ = t.send(string_data);
                    }
                    Err(_e)=>{

                    }
                }
            }
        });

        Self { receiver: r, value: None }
    }

    pub fn subscribe(&mut self)->T
    {
        let get_string = self.receiver.recv().unwrap();
        let value = T::from_packet(get_string);

        self.value = Some(value.clone());

        value
    }
}