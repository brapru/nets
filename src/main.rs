mod app;
mod os;
mod ui;

use app::{App, FilterMode};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // FIXME: Add cli arguments to take a tick rate
    let tick_rate = Duration::from_millis(1000);

    let app = Arc::new(Mutex::new(App::new()));

    let cloned_app = Arc::clone(&app);

    try_main(&cloned_app, tick_rate).await?;

    Ok(())
}

async fn try_main(
    app: &Arc<Mutex<App>>,
    tick_rate: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let last_tick = Instant::now();

    loop {
        let mut app = app.lock().await;

        terminal.draw(|f| ui::draw_ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.filter.mode {
                    FilterMode::Normal => match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
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
            break;
        }
    }

    // restore terminal
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}
