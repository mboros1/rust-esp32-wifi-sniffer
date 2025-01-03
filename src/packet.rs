use std::collections::HashSet;
use std::fmt;
use std::io::Cursor;

/// Error types for Packet parsing
#[derive(Debug)]
pub enum PacketError {
    UnknownPacket(String),
    SSIDDecodeError(String),
    // Add other error variants as needed
}

/// Enum representing the packet name
#[derive(Clone, Debug, Default)]
pub enum PacketName {
    #[default]
    Unknown,
    // Add other packet names if needed
}

/// This Packet struct holds all the information about a decoded packet.
#[derive(Clone, Default, Debug)]
pub struct Packet {
    pub pkt_name: PacketName,
    pub pkt_type: u16,
    pub pkt_subtype: u16,
    pub to_ds: u16,
    pub frm_ds: u16,
    pub signal: i8,
    pub channel: u8,
    pub addr1: String,
    pub addr2: String,
    pub addr3: String,
    pub addr4: String,
    pub ssid: String,
    pub raw_pkt: Vec<u8>,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Packet {{\n\tpkt_name: {:?},\n\tpkt_type: {},\n\tpkt_subtype: {},\n\tto_ds: {},\n\tfrm_ds: {},\n\t\
            signal: {},\n\tchannel: {},\n\taddr1: {},\n\taddr2: {},\n\taddr3: {},\n\taddr4: {},\n\tssid: {}\n}}",
            self.pkt_name, self.pkt_type, self.pkt_subtype, self.to_ds, self.frm_ds, self.signal, self.channel,
            self.addr1, self.addr2, self.addr3, self.addr4, self.ssid
        )
    }
}

impl Packet {
    /// Parses a raw packet into a `Packet` instance.
    pub fn new(pkt: &[u8]) -> Result<Packet, PacketError> {
        let mut curs = Cursor::new(pkt);
        let signal = -30; // Replace with real signal parsing logic
        let channel = 6; // Replace with real channel parsing logic

        let pkt_name = PacketName::Unknown;
        let pkt_type = 0;
        let pkt_subtype = 0;
        let to_ds = 0;
        let frm_ds = 0;

        let addr1 = "00:00:00:00:00:00".to_string();
        let addr2 = "00:00:00:00:00:00".to_string();
        let addr3 = "00:00:00:00:00:00".to_string();
        let addr4 = "00:00:00:00:00:00".to_string();
        let ssid = "test_ssid".to_string();

        Ok(Packet {
            pkt_name,
            pkt_type,
            pkt_subtype,
            to_ds,
            frm_ds,
            signal,
            channel,
            addr1,
            addr2,
            addr3,
            addr4,
            ssid,
            raw_pkt: pkt.to_vec(),
        })
    }

    /// Returns a `HashSet` of unique MAC addresses.
    pub fn hashset_addresses(&self) -> HashSet<String> {
        HashSet::from([
            self.addr1.clone(),
            self.addr2.clone(),
            self.addr3.clone(),
            self.addr4.clone(),
        ])
    }
}
