use tui::widgets::TableState;

// use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

pub const ITEMS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];

pub struct StatefulTable<'a> {
    pub state: TableState,
    pub items: Vec<Vec<&'a str>>,
}

impl<'a> StatefulTable<'a> {
    pub fn with_items(items: Vec<Vec<&'a str>>) -> StatefulTable<'a> {
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

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub show_connection_info: bool,
    pub filter: FilterField,
    pub connections: StatefulTable<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        let tasks: Vec<Vec<&str>> = vec![
            vec![
                "TCP",
                "255.255.255.255",
                "65535",
                "255.255.255.255",
                "65535",
                "ESTABLISHED",
                "11111",
                "PROC1",
            ],
            vec![
                "TCP",
                "255.255.255.255",
                "65535",
                "255.255.255.255",
                "65535",
                "ESTABLISHED",
                "22222",
                "PROC2",
            ],
            vec![
                "TCP",
                "255.255.255.255",
                "65535",
                "255.255.255.255",
                "65535",
                "ESTABLISHED",
                "33333",
                "PROC3",
            ],
        ];

        App {
            title,
            should_quit: false,
            show_connection_info: false,
            filter: FilterField {
                input: String::new(),
                mode: FilterMode::Normal,
            },
            connections: StatefulTable::with_items(tasks),
        }
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
