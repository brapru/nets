use netstat2::{ProtocolFlags, ProtocolSocketInfo, TcpState};

use crate::app::App;

use itertools::Itertools;

pub enum AddressFamily {
    IPv4,
    IPv6,
}

pub fn get_total_sockets_protocol_count(app: &App, flags: ProtocolFlags) -> usize {
    app.connections
        .clone()
        .into_iter()
        .filter(|connection| connection.protocol_flags | flags == flags)
        .count()
}

pub fn get_total_sockets_unique_count(app: &App) -> usize {
    app.connections
        .clone()
        .into_iter()
        .unique_by(
            |connection| match connection.info.protocol_socket_info.clone() {
                ProtocolSocketInfo::Tcp(tcp_si) => tcp_si.remote_addr,
                ProtocolSocketInfo::Udp(udp_si) => udp_si.local_addr,
            },
        )
        .count()
}

pub fn get_total_sockets_state_count(app: &App, state: TcpState) -> usize {
    app.connections
        .clone()
        .into_iter()
        .filter(
            |connection| match connection.info.protocol_socket_info.clone() {
                ProtocolSocketInfo::Tcp(tcp_si) => tcp_si.state == state,
                ProtocolSocketInfo::Udp(_) => false,
            },
        )
        .count()
}

pub fn get_total_sockets_ip_count(app: &App, flags: AddressFamily) -> usize {
    app.connections
        .clone()
        .into_iter()
        .filter(|connection| match flags {
            AddressFamily::IPv4 => connection.info.local_addr().is_ipv4(),
            AddressFamily::IPv6 => connection.info.local_addr().is_ipv6(),
        })
        .count()
}
