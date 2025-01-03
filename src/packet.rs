use std::collections::HashSet;
use std::fmt::{self, Formatter};
use std::io::Cursor;

/// Error types for Packet parsing
#[derive(Debug)]
pub enum PacketError {
    UnknownPacket(String),
    SSIDDecodeError(String),
    // Add other error variants as needed
}

/// All the possible packet types for 802.11.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Default)]
pub enum Packet_Name {
    /// Management Packet. Type: 0, Subtype: 0
    Association_Request,
    /// Management Packet. Type: 0, Subtype: 1
    Association_Response,
    /// Management Packet. Type: 0, Subtype: 2
    Reassociation_Request,
    /// Management Packet. Type: 0, Subtype: 3
    Reassociation_Response,
    /// Management Packet. Type: 0, Subtype: 4
    Probe_Request,
    /// Management Packet. Type: 0, Subtype: 5
    Probe_Response,
    /// Management Packet. Type: 0, Subtype: 6
    Timing_Advertisement,
    /// Management Packet. Type: 0, Subtype: 8
    Beacon,
    /// Management Packet. Type: 0, Subtype: 9
    ATIM,
    /// Management Packet. Type: 0, Subtype: 10
    Disassociation,
    /// Management Packet. Type: 0, Subtype: 11
    Authentication,
    /// Management Packet. Type: 0, Subtype: 12
    Deauthentication,
    /// Management Packet. Type: 0, Subtype: 13
    Action,
    /// Management Packet. Type: 0, Subtype: 14
    Action_No_Ack,
    /// Control Packet. Type: 1, Subtype: 2
    Trigger,
    /// Control Packet. Type: 1, Subtype: 3
    TACK,
    /// Control Packet. Type: 1, Subtype: 4
    Beamforming_Report_Poll,
    /// Control Packet. Type: 1, Subtype: 5
    VHT_HE_NDP_Announcement,
    /// Control Packet. Type: 1, Subtype: 6
    Control_Frame_Extension,
    /// Control Packet. Type: 1, Subtype: 7
    Control_Wrapper,
    /// Control Packet. Type: 1, Subtype: 8
    Block_Ack_Request,
    /// Control Packet. Type: 1, Subtype: 9
    Block_Ack,
    /// Control Packet. Type: 1, Subtype: 10
    PS_Poll,
    /// Control Packet. Type: 1, Subtype: 11
    RTS,
    /// Control Packet. Type: 1, Subtype: 12
    CTS,
    /// Control Packet. Type: 1, Subtype: 13
    ACK,
    /// Control Packet. Type: 1, Subtype: 14
    CF_End,
    /// Control Packet. Type: 1, Subtype: 15
    CF_End_CF_ACK,
    /// Data Packet. Type: 2, Subtype: 0
    Data,
    /// Data Packet. Type: 2, Subtype: 4
    Null,
    /// Data Packet. Type: 2, Subtype: 8
    QoS_Data,
    /// Data Packet. Type: 2, Subtype: 9
    QoS_Data_CF_ACK,
    /// Data Packet. Type: 2, Subtype: 10
    QoS_Data_CF_Poll,
    /// Data Packet. Type: 2, Subtype: 11
    QoS_Data_CF_ACK_CF_Poll,
    /// Data Packet. Type: 2, Subtype: 12
    QoS_Null,
    /// Data Packet. Type: 2, Subtype: 14
    QoS_CF_Poll,
    /// Data Packet. Type: 2, Subtype: 15
    QoS_CF_ACK_CF_Poll,
    /// Extension Packet. Type: 3, Subtype: 0
    DMGBeacon,
    /// Reserved Packet which can be either a Management, Control, Data or Extension Type.
    Reserved,
    /// Unknown Packet Type and Subtype.
    #[default]
    Unknown,
}

impl fmt::Display for Packet_Name {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Packet_Name::Association_Request => write!(f, "Association Request"),
            Packet_Name::Association_Response => write!(f, "Association Response"),
            Packet_Name::Reassociation_Request => write!(f, "Reassociation Request"),
            Packet_Name::Reassociation_Response => write!(f, "Reassociation Response"),
            Packet_Name::Probe_Request => write!(f, "Probe Request"),
            Packet_Name::Probe_Response => write!(f, "Probe Response"),
            Packet_Name::Timing_Advertisement => write!(f, "Timing Advertisement"),
            Packet_Name::Beacon => write!(f, "Beacon"),
            Packet_Name::ATIM => write!(f, "ATIM"),
            Packet_Name::Disassociation => write!(f, "Disassociation"),
            Packet_Name::Authentication => write!(f, "Authentication"),
            Packet_Name::Deauthentication => write!(f, "Deauthentication"),
            Packet_Name::Action => write!(f, "Action"),
            Packet_Name::Action_No_Ack => write!(f, "Action No Ack (NACK)"),
            Packet_Name::Trigger => write!(f, "Trigger"),
            Packet_Name::TACK => write!(f, "TACK"),
            Packet_Name::Beamforming_Report_Poll => write!(f, "Beamforming Report Poll"),
            Packet_Name::VHT_HE_NDP_Announcement => write!(f, "VHT/HE NDP Announcement"),
            Packet_Name::Control_Frame_Extension => write!(f, "Control Frame Extension"),
            Packet_Name::Control_Wrapper => write!(f, "Control Wrapper"),
            Packet_Name::Block_Ack_Request => write!(f, "Block Ack Request"),
            Packet_Name::Block_Ack => write!(f, "Block Ack"),
            Packet_Name::PS_Poll => write!(f, "PS-Poll"),
            Packet_Name::RTS => write!(f, "RTS"),
            Packet_Name::CTS => write!(f, "CTS"),
            Packet_Name::ACK => write!(f, "ACK"),
            Packet_Name::CF_End => write!(f, "CF-End"),
            Packet_Name::CF_End_CF_ACK => write!(f, "CF-End + CF-ACK"),
            Packet_Name::Data => write!(f, "Data"),
            Packet_Name::Null => write!(f, "Null (no data)"),
            Packet_Name::QoS_Data => write!(f, "QoS Data"),
            Packet_Name::QoS_Data_CF_ACK => write!(f, "QoS Data + CF-ACK"),
            Packet_Name::QoS_Data_CF_Poll => write!(f, "QoS Data + CF-Poll"),
            Packet_Name::QoS_Data_CF_ACK_CF_Poll => write!(f, "QoS Data + CF-ACK + CF-Poll"),
            Packet_Name::QoS_Null => write!(f, "QoS Null (no data)"),
            Packet_Name::QoS_CF_Poll => write!(f, "QoS CF-Poll (no data)"),
            Packet_Name::QoS_CF_ACK_CF_Poll => write!(f, "QoS CF-ACK + CF-Poll (no data)"),
            Packet_Name::DMGBeacon => write!(f, "DMG Beacon"),
            Packet_Name::Reserved => write!(f, "Reserved"),
            Packet_Name::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Packet_Name {
    pub fn new(pt: u16, ps: u16) -> Self {
        match (pt, ps) {
            (0, 0) => Packet_Name::Association_Request,
            (0, 1) => Packet_Name::Association_Response,
            (0, 2) => Packet_Name::Reassociation_Request,
            (0, 3) => Packet_Name::Reassociation_Response,
            (0, 4) => Packet_Name::Probe_Request,
            (0, 5) => Packet_Name::Probe_Response,
            (0, 6) => Packet_Name::Timing_Advertisement,
            (0, 7) => Packet_Name::Reserved,
            (0, 8) => Packet_Name::Beacon,
            (0, 9) => Packet_Name::ATIM,
            (0, 10) => Packet_Name::Disassociation,
            (0, 11) => Packet_Name::Authentication,
            (0, 12) => Packet_Name::Deauthentication,
            (0, 13) => Packet_Name::Action,
            (0, 14) => Packet_Name::Action_No_Ack,
            (0, 15) => Packet_Name::Reserved,
            (1, 0) => Packet_Name::Reserved,
            (1, 1) => Packet_Name::Reserved,
            (1, 2) => Packet_Name::Trigger,
            (1, 3) => Packet_Name::TACK,
            (1, 4) => Packet_Name::Beamforming_Report_Poll,
            (1, 5) => Packet_Name::VHT_HE_NDP_Announcement,
            (1, 6) => Packet_Name::Control_Frame_Extension,
            (1, 7) => Packet_Name::Control_Wrapper,
            (1, 8) => Packet_Name::Block_Ack_Request,
            (1, 9) => Packet_Name::Block_Ack,
            (1, 10) => Packet_Name::PS_Poll,
            (1, 11) => Packet_Name::RTS,
            (1, 12) => Packet_Name::CTS,
            (1, 13) => Packet_Name::ACK,
            (1, 14) => Packet_Name::CF_End,
            (1, 15) => Packet_Name::CF_End_CF_ACK,
            (2, 0) => Packet_Name::Data,
            (2, 1) => Packet_Name::Reserved,
            (2, 2) => Packet_Name::Reserved,
            (2, 3) => Packet_Name::Reserved,
            (2, 4) => Packet_Name::Null,
            (2, 5) => Packet_Name::Reserved,
            (2, 6) => Packet_Name::Reserved,
            (2, 7) => Packet_Name::Reserved,
            (2, 8) => Packet_Name::QoS_Data,
            (2, 9) => Packet_Name::QoS_Data_CF_ACK,
            (2, 10) => Packet_Name::QoS_Data_CF_Poll,
            (2, 11) => Packet_Name::QoS_Data_CF_ACK_CF_Poll,
            (2, 12) => Packet_Name::QoS_Null,
            (2, 13) => Packet_Name::Reserved,
            (2, 14) => Packet_Name::QoS_CF_Poll,
            (2, 15) => Packet_Name::QoS_CF_ACK_CF_Poll,
            (3, 0) => Packet_Name::DMGBeacon,
            _ => Packet_Name::Unknown,
        }
    }
}

/// This Packet struct holds all the information about a decoded packet.
#[derive(Clone, Default, Debug)]
pub struct Packet {
    /// Packet name determined from the pkt_type and pkt_subtype.
    pub pkt_name: Packet_Name,
    /// Packet type tell the type of packet it is.
    /// 0 = Management Packet
    /// 1 = Control Packet
    /// 2 = Data Packet
    pub pkt_type: u16,
    /// Packet subtype tell the subtype of packet it is.
    /// For more information visit <https://community.cisco.com/t5/wireless-mobility-knowledge-base/802-11-frames-a-starter-guide-to-learn-wireless-sniffer-traces/ta-p/3110019>
    pub pkt_subtype: u16,
    /// The toDS field tells if packet was sent from a device to the AP station.
    /// 0 = False
    /// 1 = True
    pub to_ds: u16,
    /// The FromDS field tells if packet was sent from a AP station to the device.
    /// 0 = False
    /// 1 = True
    pub frm_ds: u16,
    /// Signal from the RadioTap Header.
    pub signal: i8,
    /// Channel that the packet was sniffed on.
    pub channel: u8,
    /// Destination Address (DA) : Final recipient of the frame
    pub addr1: String,
    /// Source Address (SA) : Original source of the frame
    pub addr2: String,
    /// Receiver Address (RA) : Immediate receiver of the frame.
    pub addr3: String,
    /// Transmitter Address (TA) : Immediate sender of the frame.
    pub addr4: String,
    /// Service Set Identifier.
    /// Will be Base64 Encoded if the ssid is not utf-8 with a b64- at the beginning.
    pub ssid: String,
    /// The Raw Packet.
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
    pub fn new(pkt: &[u8]) -> Result<Packet, PacketError> {
        let mut curs: Cursor<&[u8]> = Cursor::new(pkt);

        // Parsing the RadioTap Header to get the Signal and Channel.
        let (signal, channel) = parse_rtap(&mut curs)?;

        // Parsing the Frame Control.
        let (pkt_type, pkt_subtype, to_ds, frm_ds, pkt_name) = frame_control(&mut curs)?;

        // Parsing the Mac
        let (addr1, addr2, addr3, addr4, ssid) = match pkt_type {
            0 => match pkt_subtype {
                0 => get_macs_and_ssid(&mut curs, 4),
                4 => get_macs_and_ssid(&mut curs, 2),
                8 | 5 => get_macs_and_ssid(&mut curs, 14),
                1..=3 | 7 | 9..=15 => get_macs(&mut curs, 3),
                _ => {
                    return Err(PacketError::UnknownPacket(format!(
                        "Packet Type: {pkt_type}, Subtype: {pkt_subtype}"
                    )))
                }
            },
            1 => match pkt_subtype {
                8..=9 | 11..=13 => get_macs(&mut curs, 2),
                0..=7 | 10 | 14 | 15 => get_macs(&mut curs, 3),
                _ => {
                    return Err(PacketError::UnknownPacket(format!(
                        "Packet Type: {pkt_type}, Subtype: {pkt_subtype}"
                    )))
                }
            },
            2 => {
                if to_ds == 1 && frm_ds == 1 {
                    get_macs(&mut curs, 4)
                } else {
                    get_macs(&mut curs, 3)
                }
            }
            3 => match pkt_subtype {
                0 => {
                    curs.set_position(curs.position() + 2);
                    let mac: String = read_mac(&mut curs);
                    (
                        none_address(),
                        mac,
                        none_address(),
                        none_address(),
                        none_address(),
                    )
                }
                _ => {
                    return Err(PacketError::UnknownPacket(format!(
                        "Packet Type: {pkt_type}, Subtype: {pkt_subtype}"
                    )))
                }
            },
            _ => {
                return Err(PacketError::UnknownPacket(format!(
                    "Packet Type: {pkt_type}, Subtype: {pkt_subtype}"
                )))
            }
        };

        let raw_pkt: Vec<u8> = pkt.to_vec();

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
            raw_pkt,
        })
    }
    /// Parses a raw packet into a `Packet` instance.

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
