mod os;

use netstat2::{AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;
    let sockets_info = netstat2::get_sockets_info(af_flags, proto_flags)?;

    for mut si in sockets_info {
        let proc_name = os::get_proc_name(si.associated_pids.pop().unwrap());

        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => println!(
                "TCP {}:{} -> {}:{} {:?} - {}",
                tcp_si.local_addr,
                tcp_si.local_port,
                tcp_si.remote_addr,
                tcp_si.remote_port,
                proc_name,
                tcp_si.state
            ),
            ProtocolSocketInfo::Udp(udp_si) => println!(
                "UDP {}:{} -> *:* {:?}",
                udp_si.local_addr, udp_si.local_port, proc_name
            ),
        }
    }

    Ok(())
}
