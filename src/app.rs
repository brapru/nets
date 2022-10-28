use netstat2::SocketInfo;
use tui::widgets::TableState;

use std::net::IpAddr;

use crate::os::get_all_socket_info;

pub const ITEMS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];

pub struct SocketInfoWithProcName {
    pub info: SocketInfo,
    pub protocol: String,
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: Option<IpAddr>,
    pub remote_port: Option<u16>,
    pub state: Option<String>,
    pub pid: u32,
    pub process_name: String,
}

impl SocketInfoWithProcName {
    pub fn make_printable_string(&self) -> Vec<String> {
        vec![
            match &self.local_addr.is_ipv4() {
                true => self.protocol.clone() + "4",
                _ => self.protocol.clone() + "6",
            },
            self.local_addr.to_string(),
            self.local_port.to_string(),
            match self.remote_addr {
                Some(addr) => addr.to_string(),
                None => String::from(""),
            },
            match self.remote_port {
                Some(port) => port.to_string(),
                None => String::from(""),
            },
            match &self.state {
                Some(state) => state.to_string(),
                None => String::from(""),
            },
            self.pid.to_string(),
            self.process_name.clone(),
        ]
    }
}

pub struct StatefulTable {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
}

impl<'a> StatefulTable {
    pub fn with_items(items: Vec<Vec<String>>) -> StatefulTable {
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

pub enum FilterMode {
    Normal,
    Typing,
}

pub struct FilterField {
    pub input: String,
    pub mode: FilterMode,
}

pub struct App {
    pub should_quit: bool,
    pub show_connection_info: bool,
    pub filter: FilterField,
    pub connections: StatefulTable,
    pub socket_info: Vec<SocketInfoWithProcName>,
    is_paused: bool,
}

impl App {
    pub fn new() -> App {
        let mut initial_connections = get_all_socket_info().unwrap();
        initial_connections.sort_by(|a, b| a.local_port.cmp(&b.local_port).reverse());

        let printable = initial_connections
            .iter()
            .map(|f| f.make_printable_string())
            .collect::<Vec<Vec<String>>>();

        App {
            should_quit: false,
            show_connection_info: false,
            is_paused: false,
            filter: FilterField {
                input: String::new(),
                mode: FilterMode::Normal,
            },
            connections: StatefulTable::with_items(printable),
            socket_info: Vec::new(),
        }
    }

    pub fn update_connections(&mut self) {
        let mut initial_connections = get_all_socket_info().unwrap();
        initial_connections.sort_by(|a, b| a.local_port.cmp(&b.local_port).reverse());

        let printable = initial_connections
            .iter()
            .map(|f| f.make_printable_string())
            .collect::<Vec<Vec<String>>>();

        self.connections.items = printable;
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

    pub fn on_key(&mut self, c: char) {
        match c {
            '/' => {
                self.filter.mode = FilterMode::Typing;
            }
            'j' => {
                self.on_down();
            }
            'k' => {
                self.on_up();
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
