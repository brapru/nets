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
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // FIXME: Add cli arguments to take a tick rate
    let tick_rate = Duration::from_millis(250);

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

    loop {
        let mut app = app.lock().await;

        terminal.draw(|f| ui::draw_ui(f, &mut app))?;

        if crossterm::event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match app.filter.mode {
                    FilterMode::Normal => match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char(c) => app.on_key(c),
                        KeyCode::Up => app.on_up(),
                        KeyCode::Down => app.on_down(),
                        KeyCode::Left => app.on_left(),
                        KeyCode::Right => app.on_right(),
                        KeyCode::Tab => app.on_right(),
                        _ => {}
                    },
                    FilterMode::Typing => match key.code {
                        // FIXME: This should apply the filter
                        KeyCode::Enter => {
                            app.update_regex();
                            app.filter.mode = FilterMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.filter.input.push(c);
                            app.update_regex();
                        }
                        KeyCode::Backspace => {
                            app.filter.input.pop();
                            app.update_regex();
                        }
                        KeyCode::Esc => {
                            app.update_regex();
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
