use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset},
};
use std::{
    io::{Result, stdout},
    process, thread,
    time::Duration,
};

use lorenz::lorenz;

const SIGMA: f64 = 10.0;
const RHO: f64 = 28.0;
const BETA: f64 = 8.0 / 3.0;

fn main() -> Result<()> {

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let dt = 0.01;
    let num_steps = 10000;

    let mut x = 0.0;
    let mut y = 1.0;
    let mut z = 1.05;
    let mut points: Vec<(f64, f64)> = Vec::new();

    let mut step = 0;

    loop {
        if step < num_steps {
            let (dx, dy, dz) = lorenz(x, y, z, SIGMA, RHO, BETA).unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                process::exit(1);
            });
            x += dx * dt;
            y += dy * dt;
            z += dz * dt;
            points.push((x, z));
            step += 1;
        } else {
            break;
        }

        terminal.draw(|f| {
            let size = f.area();
            let chart_area = Rect {
                x: 0,
                y: 0,
                width: size.width,
                height: size.height,
            };

            let dataset = Dataset::default()
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&points);

            let chart = Chart::new(vec![dataset])
                // .block(Block::default().borders(Borders::ALL).title("Lorenz Attractor (X-Z)"))
                .x_axis(
                    Axis::default() /*.title("X")*/
                        .bounds([-20.0, 20.0]),
                )
                .y_axis(
                    Axis::default() /*.title("Z")*/
                        .bounds([0.0, 50.0]),
                );

            f.render_widget(chart, chart_area);
        })?;

        // adjust delay for animation
        thread::sleep(Duration::from_millis(1));

        if event::poll(Duration::from_millis(10))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
