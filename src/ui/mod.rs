use super::app::App;
use super::app::FilterMode;
use super::app::ITEMS;

use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
    Frame,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    Terminal,
};

use unicode_width::UnicodeWidthStr;

#[derive(Default)]
pub struct TableHeaderItem<'a> {
    text: &'a str,
    width: u16,
}

pub struct TableHeader<'a> {
    items: Vec<TableHeaderItem<'a>>,
}

pub fn get_percentage_width(width: u16, percentage: f32) -> u16 {
    let padding = 3;
    let width = width - padding;
    (f32::from(width) * percentage) as u16
}

pub fn start_ui(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("nets");
    let res = start_ui_loop(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn start_ui_loop<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let last_tick = Instant::now();

    loop {
        terminal.draw(|f| draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.filter.mode {
                    FilterMode::Normal => match key.code {
                        KeyCode::Char(c) => app.on_key(c),
                        KeyCode::Up => app.on_up(),
                        KeyCode::Down => app.on_down(),
                        _ => {}
                    },
                    FilterMode::Typing => match key.code {
                        // FIXME: This should apply the filter
                        KeyCode::Enter => {}
                        KeyCode::Char(c) => {
                            app.filter.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.filter.input.pop();
                        }
                        KeyCode::Esc => {
                            app.filter.mode = FilterMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(6),
                Constraint::Length(3),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_help(f, app, chunks[0]);
    draw_filter_field(f, app, chunks[1]);
    draw_connections(f, app, chunks[2]);
}

fn draw_help<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let (msg, style) = match app.filter.mode {
        FilterMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("/", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to search on regex"),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        FilterMode::Typing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);

    f.render_widget(help_message, area);
}

fn draw_filter_field<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let input = Paragraph::new(app.filter.input.as_ref())
        .style(match app.filter.mode {
            FilterMode::Normal => Style::default(),
            FilterMode::Typing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Filter"));

    f.render_widget(input, area);

    match app.filter.mode {
        FilterMode::Normal => {}

        FilterMode::Typing => {
            f.set_cursor(area.x + app.filter.input.width() as u16 + 1, area.y + 1)
        }
    }
}

fn draw_connections<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let constraints = if app.show_connection_info {
        vec![Constraint::Percentage(75), Constraint::Percentage(25)]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let chunks = Layout::default()
        .constraints(constraints.as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    draw_connection_table(f, app, chunks[0]);

    if app.show_connection_info {
        draw_connection_info_table(f, app, chunks[1]);
    }
}

fn draw_connection_table<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    let header = TableHeader {
        items: vec![
            TableHeaderItem {
                text: "Protocol",
                width: get_percentage_width(area.width, 0.1),
            },
            TableHeaderItem {
                text: "Local Address",
                width: get_percentage_width(area.width, 0.16),
            },
            TableHeaderItem {
                text: "Local Port",
                width: get_percentage_width(area.width, 0.1),
            },
            TableHeaderItem {
                text: "Remote Address",
                width: get_percentage_width(area.width, 0.16),
            },
            TableHeaderItem {
                text: "Remote Port",
                width: get_percentage_width(area.width, 0.1),
            },
            TableHeaderItem {
                text: "State",
                width: get_percentage_width(area.width, 0.1),
            },
            TableHeaderItem {
                text: "PID",
                width: get_percentage_width(area.width, 0.1),
            },
            TableHeaderItem {
                text: "Process Name",
                width: get_percentage_width(area.width, 0.16),
            },
        ],
    };

    let formatted_header = Row::new(header.items.iter().map(|h| h.text))
        .style(Style::default().add_modifier(Modifier::BOLD));

    let rows = app.connections.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(0)
    });

    let widths = header
        .items
        .iter()
        .map(|h| Constraint::Length(h.width))
        .collect::<Vec<tui::layout::Constraint>>();

    let t = Table::new(rows)
        .header(formatted_header)
        .block(Block::default().borders(Borders::ALL).title("Connections"))
        .highlight_style(selected_style)
        .widths(&widths);

    f.render_stateful_widget(t, area, &mut app.connections.state);
}

fn draw_connection_info_table<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let connections: Vec<ListItem> = ITEMS
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
        .collect();

    let connections = List::new(connections)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Connection Info"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(connections, area);
}
