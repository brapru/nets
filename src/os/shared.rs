#[cfg(any(target_os = "macos"))]
use crate::os::macos::libproc::*;

use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

pub fn get_proc_name(pid: u32) -> String {
    let retval = get_os_proc_name(i32::try_from(pid).unwrap());

    match retval {
        Ok(name) => name,
        Err(_) => String::from("-"),
    }
}
pub fn get_all_socket_info() -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut open_sockets: Vec<Vec<String>> = Vec::new();

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags)?;

    for mut si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                let socket = vec![
                    String::from("tcp"),
                    tcp_si.local_addr.to_string(),
                    tcp_si.local_port.to_string(),
                    tcp_si.remote_addr.to_string(),
                    tcp_si.remote_port.to_string(),
                    tcp_si.state.to_string(),
                    si.associated_pids.clone().pop().unwrap().to_string(),
                    get_proc_name(si.associated_pids.pop().unwrap()),
                ];

                open_sockets.push(socket);
            }
            ProtocolSocketInfo::Udp(udp_si) => {
                let socket = vec![
                    String::from("udp"),
                    udp_si.local_addr.to_string(),
                    udp_si.local_port.to_string(),
                    String::from("-"),
                    String::from("-"),
                    String::from("-"),
                    si.associated_pids.clone().pop().unwrap().to_string(),
                    get_proc_name(si.associated_pids.pop().unwrap()),
                ];

                open_sockets.push(socket);
            }
        }
    }

    Ok(open_sockets)
}
