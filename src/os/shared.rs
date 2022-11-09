use crate::app::SocketInfoWithProcName;

#[cfg(any(target_os = "macos"))]
use crate::os::macos::libproc::*;

#[cfg(any(target_os = "linux"))]
use crate::os::linux::proc::*;

use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags};

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
        open_sockets.push(SocketInfoWithProcName::new(
            si.clone(),
            {
                match si.associated_pids.pop() {
                    Some(pid) => get_proc_name(pid),
                    _ => "-".to_string()
                }
            }
        ));
    }

    Ok(open_sockets)
}
