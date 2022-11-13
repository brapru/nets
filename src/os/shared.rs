use netstat2::{
    get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, SocketInfo,
};
use regex::Regex;

#[cfg(any(target_os = "macos"))]
use crate::os::macos::libproc::*;

#[cfg(any(target_os = "linux"))]
use crate::os::linux::proc::*;

#[derive(Clone)]
pub struct SocketInfoWithProcName {
    pub info: SocketInfo,
    pub process_name: String,
    pub printable_string: Vec<String>,
    pub protocol_flags: ProtocolFlags,
}

impl SocketInfoWithProcName {
    pub fn new(info: SocketInfo, name: String) -> SocketInfoWithProcName {
        match &info.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => SocketInfoWithProcName {
                protocol_flags: ProtocolFlags::TCP,
                info: info.clone(),
                process_name: name.clone(),
                printable_string: vec![
                    match tcp_si.local_addr.is_ipv4() {
                        true => String::from("tcp4"),
                        _ => String::from("tcp6"),
                    },
                    tcp_si.local_addr.to_string(),
                    tcp_si.local_port.to_string(),
                    tcp_si.remote_addr.to_string(),
                    tcp_si.remote_port.to_string(),
                    tcp_si.state.to_string(),
                    match info.clone().associated_pids.pop() {
                        Some(pid) => pid.to_string(),
                        _ => "-".to_string(),
                    },
                    name,
                ],
            },
            ProtocolSocketInfo::Udp(udp_si) => SocketInfoWithProcName {
                protocol_flags: ProtocolFlags::UDP,
                info: info.clone(),
                process_name: name.clone(),
                printable_string: vec![
                    match udp_si.local_addr.is_ipv4() {
                        true => String::from("udp4"),
                        _ => String::from("udp6"),
                    },
                    udp_si.local_addr.to_string(),
                    udp_si.local_port.to_string(),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    match info.clone().associated_pids.pop() {
                        Some(pid) => pid.to_string(),
                        _ => "-".to_string(),
                    },
                    name,
                ],
            },
        }
    }

    pub fn should_print(&self, regex: &Option<Regex>) -> bool {
        if regex.is_none() {
            return true;
        }

        self.printable_string
            .iter()
            .any(|cell| regex.as_ref().unwrap().is_match(cell))
    }
}

pub fn get_proc_name(pid: u32) -> String {
    let retval = get_os_proc_name(i32::try_from(pid).unwrap());

    match retval {
        Ok(name) => name,
        Err(_) => String::from("-"),
    }
}

pub fn get_all_socket_info(
    protocol: ProtocolFlags,
) -> Result<Vec<SocketInfoWithProcName>, Box<dyn std::error::Error>> {
    let mut open_sockets: Vec<SocketInfoWithProcName> = Vec::new();

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let sockets_info = get_sockets_info(af_flags, protocol)?;

    for mut si in sockets_info {
        open_sockets.push(SocketInfoWithProcName::new(si.clone(), {
            match si.associated_pids.pop() {
                Some(pid) => get_proc_name(pid),
                _ => "-".to_string(),
            }
        }));
    }

    Ok(open_sockets)
}
