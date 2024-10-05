use super::message::FLSMsg;
use super::utils::name_to_port;
use std::sync::mpsc::Sender;

pub struct Publisher<T : FLSMsg>
{
    sender : Sender<String>,
    value : Option<T>
}

impl<T : FLSMsg> Publisher<T> {
    pub fn new(node_name : String, topic_name : String)->Publisher<T>
    {
        let (t, r) = std::sync::mpsc::channel::<String>();

        let port = name_to_port(node_name.clone());
        let dest_port = name_to_port(topic_name);

        std::thread::spawn(move ||{
            let sock = std::net::UdpSocket::bind(format!("127.0.0.1:{}", port)).unwrap();
            let dest_addr = format!("127.0.0.1:{}", dest_port);

            loop {
                let v = r.recv().unwrap();

                match sock.send_to(v.as_bytes(), dest_addr.as_str()) {
                    Ok(_s)=>{

                    }
                    Err(_e)=>{

                    }
                }
            }
        });

        Self {sender: t, value : None}
    }

    pub fn publish(&mut self, value : T)
    {
        self.value = Some(value.clone());
        let _ = self.sender.send(value.create_packet());
    }
}