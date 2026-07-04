use std::time::{Duration, Instant};

use app::App;
use rand::RngExt;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::Color,
};

mod app;
mod grid;

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();

    run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let mut area = terminal.get_frame().area();
    let mut app = App::new(area.width as usize, area.height as usize);
    let mut last_star = Instant::now();
    let mut rnd = rand::rng();
    loop {
        let _ = terminal.draw(|frame| {
            let new_area = frame.area();
            if new_area.width != area.width || new_area.height != area.height {
                area = new_area;
                let color = Color::Rgb(
                    rnd.random_range(50..=255),
                    rnd.random_range(50..=255),
                    rnd.random_range(50..=255),
                );

                app = App::new(area.width as usize, area.height as usize);
                app.set_cell_color(color);
                app.randomize();
            }

            if last_star.elapsed() >= Duration::from_mins(3) {
                app.rerandomize_period(&mut last_star, 3);
            }

            app.update(frame);
        });

        if handle_events(&mut app)? {
            break Ok(());
        }
        app.rerandomize_period(&mut last_star, 3);
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
            KeyCode::Char('r') => {
                app.randomize();
            }
            KeyCode::Char('n') => app.one_step(),
            KeyCode::Char('h') => app.toggle_help(),

            _ => {}
        },

        _ => {}
    }

    Ok(false)
}
