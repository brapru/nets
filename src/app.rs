use netstat2::ProtocolFlags;
use regex::Regex;
use tui::widgets::TableState;

use crate::os::{get_all_socket_info, SocketInfoWithProcName};

pub const ITEMS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];

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

    pub fn first(&mut self) {
        self.state.select(Some(0));
    }

    pub fn last(&mut self) {
        self.state.select(Some(self.items.len() - 1));
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

impl StatefulTabs {
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
    pub show_help: bool,
    pub filter: FilterField,
    pub tabs: StatefulTabs,
    pub connections: Vec<SocketInfoWithProcName>,
    pub connection_table: StatefulTable,
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
            show_help: false,
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
            connections: initial_connections.clone(),
            connection_table: StatefulTable::with_items(initial_connections),
            printable_lines: Vec::new(),
        }
    }

    pub fn update_connections(&mut self) {
        let connections: Vec<SocketInfoWithProcName>;

        if self.is_paused() {
            connections = self.connections.clone();
        } else {
            connections = get_all_socket_info(ProtocolFlags::TCP | ProtocolFlags::UDP).unwrap();
            self.connections = connections.clone();
        };

        let mut filtered: Vec<SocketInfoWithProcName> = connections
            .into_iter()
            .filter(|connection| {
                connection.protocol_flags | self.tabs.selected_protocol()
                    == self.tabs.selected_protocol()
            })
            .collect();

        filtered.sort_by(|a, b| a.info.local_port().cmp(&b.info.local_port()).reverse());

        self.connection_table.items = filtered
            .into_iter()
            .filter(|connection| connection.should_print(&self.filter.regex))
            .collect();
    }

    pub fn update_regex(&mut self) {
        if self.filter.input.is_empty() {
            self.filter.regex = None;
            return;
        }

        self.filter.regex = Some(Regex::new(&regex::escape(&self.filter.input)).unwrap());
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn on_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn on_show_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn on_up(&mut self) {
        self.connection_table.previous();
    }

    pub fn on_down(&mut self) {
        self.connection_table.next();
    }

    pub fn on_first(&mut self) {
        self.connection_table.first();
    }

    pub fn on_last(&mut self) {
        self.connection_table.last();
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
            'c' => {
                self.filter.input.clear();
                self.update_regex();
            }
            'G' => {
                self.on_last();
            }
            'h' => {
                self.on_left();
            }
            'j' => {
                self.on_down();
            }
            'k' => {
                self.on_up();
            }
            'l' => {
                self.on_right();
            }
            'i' => {
                self.show_connection_info = !self.show_connection_info;
            }
            _ => {}
        }
    }
}
