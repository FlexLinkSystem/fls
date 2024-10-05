use serde::{Deserialize, Serialize};
use serde_json;

pub trait FLSMsg
where
    Self: Serialize + for<'de> Deserialize<'de> + Sized,
{
    fn create_packet(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_packet(packet: String) -> Self {
        serde_json::from_str(&packet).unwrap()
    }
}