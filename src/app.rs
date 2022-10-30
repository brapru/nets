use netstat2::{ProtocolFlags, ProtocolSocketInfo, SocketInfo};
use regex::Regex;
use tui::widgets::TableState;

use crate::os::get_all_socket_info;

pub const ITEMS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];

pub struct SocketInfoWithProcName {
    pub info: SocketInfo,
    pub process_name: String,
    pub printable_string: Vec<String>,
}

impl SocketInfoWithProcName {
    pub fn new(info: SocketInfo, name: String) -> SocketInfoWithProcName {
        match &info.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => SocketInfoWithProcName {
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
                    info.associated_pids.clone().pop().unwrap().to_string(),
                    name.clone(),
                ],
            },
            ProtocolSocketInfo::Udp(udp_si) => SocketInfoWithProcName {
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
                    info.associated_pids.clone().pop().unwrap().to_string(),
                    name.clone(),
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

pub struct StatefulTable {
    pub state: TableState,
    pub items: Vec<SocketInfoWithProcName>,
}

impl StatefulTable {
    pub fn with_items(items: Vec<SocketInfoWithProcName>) -> StatefulTable {
        StatefulTable {
            state: TableState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct StatefulTabItem {
    pub title: String,
    pub protocol: ProtocolFlags,
}

pub struct StatefulTabs {
    pub items: Vec<StatefulTabItem>,
    pub index: usize,
}

impl<'a> StatefulTabs {
    pub fn with_items(items: Vec<StatefulTabItem>) -> StatefulTabs {
        StatefulTabs { items, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.items.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.items.len() - 1;
        }
    }

    pub fn selected_protocol(&self) -> ProtocolFlags {
        self.items[self.index].protocol
    }
}

pub enum FilterMode {
    Normal,
    Typing,
}

pub struct FilterField {
    pub input: String,
    pub mode: FilterMode,
    pub regex: Option<Regex>,
}

pub struct App {
    pub should_quit: bool,
    pub show_connection_info: bool,
    pub filter: FilterField,
    pub tabs: StatefulTabs,
    pub connections: StatefulTable,
    pub socket_info: Vec<SocketInfoWithProcName>,
    pub printable_lines: Vec<Vec<String>>,
    is_paused: bool,
}

impl App {
    pub fn new() -> App {
        let mut initial_connections =
            get_all_socket_info(ProtocolFlags::TCP | ProtocolFlags::UDP).unwrap();
        initial_connections.sort_by(|a, b| a.info.local_port().cmp(&b.info.local_port()).reverse());

        App {
            should_quit: false,
            show_connection_info: false,
            is_paused: false,
            filter: FilterField {
                input: String::new(),
                mode: FilterMode::Normal,
                regex: None,
            },
            tabs: StatefulTabs::with_items(vec![
                StatefulTabItem {
                    title: String::from("All"),
                    protocol: ProtocolFlags::TCP | ProtocolFlags::UDP,
                },
                StatefulTabItem {
                    title: String::from("TCP"),
                    protocol: ProtocolFlags::TCP,
                },
                StatefulTabItem {
                    title: String::from("UDP"),
                    protocol: ProtocolFlags::UDP,
                },
            ]),
            connections: StatefulTable::with_items(initial_connections),
            socket_info: Vec::new(),
            printable_lines: Vec::new(),
        }
    }

    pub fn update_connections(&mut self) {
        let mut updated = get_all_socket_info(self.tabs.selected_protocol()).unwrap();
        updated.sort_by(|a, b| a.info.local_port().cmp(&b.info.local_port()).reverse());

        self.connections.items = updated
            .into_iter()
            .filter(|connection| connection.should_print(&self.filter.regex))
            .collect();
    }

    pub fn update_regex(&mut self) {
        self.filter.regex = Some(Regex::new(&regex::escape(&self.filter.input)).unwrap());
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn on_up(&mut self) {
        self.connections.previous();
    }

    pub fn on_down(&mut self) {
        self.connections.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            '/' => {
                self.filter.mode = FilterMode::Typing;
            }
            'h' => {
                if self.is_paused() {
                    return;
                }
                self.on_left();
            }
            'j' => {
                self.on_down();
            }
            'k' => {
                self.on_up();
            }
            'l' => {
                if self.is_paused() {
                    return;
                }
                self.on_right();
            }
            'p' => {
                self.is_paused = !self.is_paused;
            }
            'q' => {
                self.should_quit = true;
            }
            'i' => {
                self.show_connection_info = !self.show_connection_info;
            }
            _ => {}
        }
    }
}
