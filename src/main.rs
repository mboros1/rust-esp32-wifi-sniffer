mod packet;

use packet::Packet;
use packet::PacketError;

use std::os::raw::c_void;

use esp_idf_svc::{
    hal::prelude::Peripherals,
    sys::{
        wifi_promiscuous_pkt_t, wifi_promiscuous_pkt_type_t,
        wifi_promiscuous_pkt_type_t_WIFI_PKT_CTRL, wifi_promiscuous_pkt_type_t_WIFI_PKT_DATA,
        wifi_promiscuous_pkt_type_t_WIFI_PKT_MGMT, wifi_promiscuous_pkt_type_t_WIFI_PKT_MISC,
    },
};

// rename constants to something more linter and eyeball friendly
const WIFI_PKT_MGMT: u32 = wifi_promiscuous_pkt_type_t_WIFI_PKT_MGMT;
const WIFI_PKT_DATA: u32 = wifi_promiscuous_pkt_type_t_WIFI_PKT_DATA;
const WIFI_PKT_CTRL: u32 = wifi_promiscuous_pkt_type_t_WIFI_PKT_CTRL;
const WIFI_PKT_MISC: u32 = wifi_promiscuous_pkt_type_t_WIFI_PKT_MISC;

// Callback for promiscuous mode
extern "C" fn wifi_sniffer_packet_handler(
    buf: *mut c_void,
    packet_type: wifi_promiscuous_pkt_type_t,
) {
    if buf.is_null() {
        return;
    }

    unsafe {
        let packet = &*(buf as *const wifi_promiscuous_pkt_t);
        let ctrl = &packet.rx_ctrl;

        let raw_pkt =
            std::slice::from_raw_parts(packet.payload.as_ptr(), packet.rx_ctrl.sig_len() as usize);

        match Packet::new(raw_pkt) {
            Ok(decoded_packet) => {
                println!("Decoded Packet: {:?}", decoded_packet);
            }
            Err(e) => {
                println!("Error decoding packet: {:?}", e);
            }
        }

        match packet_type {
            WIFI_PKT_MGMT => {
                let payload_ptr = packet.payload.as_ptr() as *const u8;
                let payload_len = ctrl.sig_len() as usize;

                let payload = std::slice::from_raw_parts(payload_ptr, payload_len);

                if payload_len > 0 && payload[0] == 0x80 {
                    println!("Beacon frame captured: RSSI={}", ctrl.rssi());
                    parse_beacon_packet(payload_ptr, payload_len);
                }
            }
            WIFI_PKT_DATA => {
                println!(
                    "Data frame captured: Length={}, RSSI={}",
                    ctrl.sig_len(),
                    ctrl.rssi()
                );
            }
            WIFI_PKT_CTRL => {
                println!(
                    "Control frame captured: Length={}, RSSI={}",
                    ctrl.sig_len(),
                    ctrl.rssi()
                );
            }
            WIFI_PKT_MISC => {
                println!(
                    "Miscellaneous frame captured: Length={}, RSSI={}",
                    ctrl.sig_len(),
                    ctrl.rssi()
                );
            }
            _ => {
                println!("Unknown frame type: {}", packet_type as u32);
            }
        }
    }
}

// Function to parse and log beacon packet details
fn parse_beacon_packet(payload: *const u8, len: usize) {
    unsafe {
        let slice = std::slice::from_raw_parts(payload, len);
        println!("Beacon Packet Data: {:?}", slice);
    }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    //let _netif = esp_idf_svc::netif::EspNetif::new(esp_idf_svc::netif::NetifStack::Sta).unwrap();

    let peripherals = Peripherals::take().unwrap();
    let modem = peripherals.modem;
    let sysloop = esp_idf_svc::eventloop::EspSystemEventLoop::take().unwrap();
    let nvs = esp_idf_svc::nvs::EspDefaultNvsPartition::take().ok();

    let mut wifi = esp_idf_svc::wifi::EspWifi::new(modem, sysloop, nvs).unwrap();

    wifi.start().unwrap();

    let wifi_driver = wifi.driver_mut();

    unsafe {
        esp_idf_svc::sys::esp_wifi_set_promiscuous_rx_cb(Some(wifi_sniffer_packet_handler));
    }
    wifi_driver.set_promiscuous(true).unwrap();

    log::info!("Hello, world!");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
