use crate::app::SocketInfoWithProcName;

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
pub fn get_all_socket_info() -> Result<Vec<SocketInfoWithProcName>, Box<dyn std::error::Error>> {
    let mut open_sockets: Vec<SocketInfoWithProcName> = Vec::new();

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags)?;

    for mut si in sockets_info {
        match &si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                open_sockets.push(SocketInfoWithProcName {
                    info: si.clone(),
                    protocol: String::from("tcp"),
                    local_addr: tcp_si.local_addr,
                    local_port: tcp_si.local_port,
                    remote_addr: Some(tcp_si.remote_addr),
                    remote_port: Some(tcp_si.remote_port),
                    state: Some(tcp_si.state.to_string()),
                    pid: si.clone().associated_pids.pop().unwrap(),
                    process_name: get_proc_name(si.associated_pids.pop().unwrap()),
                });
            }
            ProtocolSocketInfo::Udp(udp_si) => {
                open_sockets.push(SocketInfoWithProcName {
                    info: si.clone(),
                    protocol: String::from("udp"),
                    local_addr: udp_si.local_addr,
                    local_port: udp_si.local_port,
                    remote_addr: None,
                    remote_port: None,
                    state: None,
                    pid: si.clone().associated_pids.pop().unwrap(),
                    process_name: get_proc_name(si.associated_pids.pop().unwrap()),
                });
            }
        }
    }
    Ok(open_sockets)
}
