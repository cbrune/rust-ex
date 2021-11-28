//! Networking utilities

use std::fs;
use std::io::{self, BufRead};
use std::net::Ipv4Addr;

use crate::prelude::*;

/// A network gateway
#[derive(Debug, Clone)]
pub struct Gateway {
    /// Name of the network interface
    pub device: String,

    /// The next hop neighbor IP address
    pub next_hop: Ipv4Addr,
}

/// Get a NetworkInterface object given an interface name
pub fn get_interface(name: &str) -> Result<NetworkInterface, AppError> {
    let interface = pnet::datalink::interfaces()
        .into_iter()
        .find(|intf| intf.name == name)
        .ok_or_else(|| AppError::InterfaceNotFound(name.to_owned()))?;

    Ok(interface)
}

/// Get the current default IPv4 gateway
pub fn get_default_gw() -> Result<Option<Gateway>, AppError> {
    let proc_file = fs::File::open("/proc/net/route")?;
    let mut lines = io::BufReader::new(proc_file).lines();

    // Lines of /proc/net/route look like
    // Iface   Destination     Gateway         Flags   RefCnt  Use     Metric  Mask            MTU     Window  IRTT
    // br0     00000000        01000A0A        0003    0       0       100     00000000        0       0       0
    // br0     00000A0A        00000000        0001    0       0       0       00FCFFFF        0       0       0
    // ....
    // We want the one with Destination = 0 and Gateway != 0.

    // skip first line
    lines.next();

    let mut result = None;
    for line in lines {
        let input = line?;
        let field_vector: Vec<&str> = input.split_whitespace().collect();
        let intf = field_vector[0];
        if let Ok(destination) = u32::from_str_radix(field_vector[1], 16) {
            if destination == 0 {
                if let Ok(gateway) = u32::from_str_radix(field_vector[2], 16) {
                    result = Some(Gateway {
                        device: intf.to_owned(),
                        next_hop: Ipv4Addr::from(u32::from_be(gateway)),
                    });
                    break;
                }
            }
        }
    }

    Ok(result)
}

// ping IPv4 address
