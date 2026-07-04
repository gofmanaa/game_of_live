use std::time::{Duration, Instant};

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::grid::Grid;

pub struct App {
    pub grid: Grid,
    last_tick: Instant,
    paused: bool,
    show_help: bool,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::random(width, height),
            last_tick: Instant::now(),
            paused: false,
            show_help: false,
        }
    }

    pub fn randomize(&mut self) {
        self.grid = Grid::random(self.grid.width, self.grid.height);
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn one_step(&mut self) {
        self.paused = true;
        self.grid.step();
    }

    pub fn rerandomize_period(&mut self, last_start: &mut Instant, mins: u64) {
        if last_start.elapsed() >= Duration::from_mins(mins) {
            self.randomize();
            *last_start = Instant::now();
        }
    }

    pub fn update(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let mut lines = Vec::new();

        for y in 0..self.grid.height {
            let mut line = String::new();

            for x in 0..self.grid.width {
                line.push(if self.grid.alive(x, y) { '█' } else { ' ' });
            }

            lines.push(Line::from(line));
        }

        let title = if self.is_paused() {
            "Game of Life (Paused)"
        } else {
            "Game of Life"
        };

        let paragraph = Paragraph::new(lines)
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .title(title)
                    .style(Style::new().green()),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black));

        frame.render_widget(paragraph, area);

        if self.show_help {
            let help_text = vec![
                Line::from(" Keyboard Controls "),
                Line::from("-------------------"),
                Line::from(" q      : Quit"),
                Line::from(" Space  : Toggle pause"),
                Line::from(" r      : Randomize"),
                Line::from(" n      : Step over"),
            ];
            let help_width = 40;
            let help_height = help_text.len() as u16 + 2;
            let help_area = Rect {
                x: (area.width.saturating_sub(help_width)) / 2,
                y: (area.height.saturating_sub(help_height)) / 2,
                width: help_width,
                height: help_height,
            };
            let help_area = help_area.intersection(area);
            frame.render_widget(Clear, help_area);
            frame.render_widget(
                Paragraph::new(help_text)
                    .block(Block::default().borders(Borders::ALL).title(" Help "))
                    .style(Style::default().fg(Color::Green).bg(Color::Black)),
                help_area,
            );
        }

        if self.is_paused() {
            return;
        }

        if self.last_tick.elapsed() >= Duration::from_millis(100) {
            self.grid.step();
            self.last_tick = Instant::now();
        }
    }
}
