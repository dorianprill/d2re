/// decoded D2GS packet data type
use std::fmt;

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

#[allow(dead_code)]
pub struct D2GSPacket {
    pub data: Vec<u8>,
}

impl AsBytes for D2GSPacket {
    fn as_bytes(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl fmt::Display for D2GSPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(0x{:02X}): [", self.packet_id())?;
        for v in &self.data {
            write!(f, "{:02X},", v)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl D2GSPacket {
    pub fn packet_id(&self) -> u8 {
        self.data[0]
    }
    pub fn payload(&self) -> &[u8] {
        &self.data[1..]
    }
}
