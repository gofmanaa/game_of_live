use std::time::Duration;

use app::App;
use ratatui::crossterm::event::{self, Event, KeyCode};

mod app;
mod grid;

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();

    run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let area = terminal.get_frame().area();
    let mut app = App::new(area.width as usize, area.height as usize);
    loop {
        terminal.draw(|frame| app.update(frame))?;
        if handle_events(&mut app)? {
            break Ok(());
        }
    }
}

fn handle_events(app: &mut App) -> std::io::Result<bool> {
    if !event::poll(Duration::from_millis(1))? {
        return Ok(false);
    }

    match event::read()? {
        Event::Key(key) if key.is_press() => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char(' ') => app.toggle_pause(),
            KeyCode::Char('r') => app.randomize(),
            KeyCode::Char('n') => app.one_step(),
            KeyCode::Char('h') => app.toggle_help(),

            _ => {}
        },

        _ => {}
    }

    Ok(false)
}
