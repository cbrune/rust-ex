//! Networking utilities

use std::fs;
use std::io::{self, BufRead};
use std::net::Ipv4Addr;
use std::process::{Command, Stdio};
use std::str::FromStr;

use crate::prelude::*;

/// A network gateway
#[derive(Debug, Clone, PartialEq)]
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

/// Get gateway associated with ISP interface
pub fn get_isp_gateway(name: &str) -> Result<Option<Gateway>, AppError> {
    let lease_file_name = format!("/tmp/dhclient.{}.leases", name);
    debug!("Trying to open {}", lease_file_name);
    let lease_file = fs::File::open(lease_file_name)?;
    let lines = io::BufReader::new(lease_file).lines();
    let last_line = lines
        .filter(|l| l.as_ref().unwrap().contains("option router"))
        .last();
    let gateway = if let Some(line) = last_line {
        let input = line?;
        debug!("Trying to parse line: {}", input);
        let fields: Vec<&str> = input.split_whitespace().collect();
        if fields[0] == "option" && fields[1] == "routers" {
            debug!("Found line: {}", input);
            Some(Gateway {
                device: name.to_owned(),
                next_hop: Ipv4Addr::from_str(fields[2].trim_end_matches(';'))?,
            })
        } else {
            None
        }
    } else {
        None
    };

    Ok(gateway)
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
        let fields: Vec<&str> = input.split_whitespace().collect();
        let intf = fields[0];
        if let Ok(destination) = u32::from_str_radix(fields[1], 16) {
            if destination == 0 {
                if let Ok(gateway) = u32::from_str_radix(fields[2], 16) {
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

/// ping IPv4 address
pub fn ping_addr(dest: &str) -> Result<bool, AppError> {
    Ok(Command::new("ping")
        .args(["-4", "-c", "2", "-w", "2", dest])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?
        .success())
}
