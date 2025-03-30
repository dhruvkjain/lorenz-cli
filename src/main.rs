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
    widgets::{Axis, Chart, Dataset},
};
use std::{
    io::{Result, stdout},
    process,
    time::Duration,
};

use lorenz_cli::lorenz;

const SIGMA: f64 = 10.0;
const RHO: f64 = 28.0;
const BETA: f64 = 8.0 / 3.0;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let axis_choice = std::env::args().nth(1).unwrap_or_else(|| "xy".to_string());

    let dt = 0.01;
    let num_steps = 10000;

    'lorenz_attractor: loop {
        let mut x = 6.0;
        let mut y = 1.0;
        let mut z = 1.05;
        let mut points: Vec<(f64, f64)> = Vec::new();

        let mut step = 0;
        // let points = generate_lorenz_points();

        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;

        while step < num_steps {
            let (dx, dy, dz) = lorenz(x, y, z, SIGMA, RHO, BETA).unwrap_or_else(|err| {
                disable_raw_mode().ok();
                execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
                eprintln!("{:?}", err);
                process::exit(1);
            });
            if dx.abs() > 1000.0 || dy.abs() > 1000.0 || dz.abs() > 1000.0 {
                disable_raw_mode()?;
                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                eprintln!("Diverging Lorenz Attractor! Exiting...");
                process::exit(1);
            }
            x += dx * dt;
            y += dy * dt;
            z += dz * dt;
            let point = match axis_choice.as_str() {
                "xy" => (x, y),
                "xz" => (x, z),
                "yz" => (y, z),
                _ => (x, y),
            };

            x_min = x_min.min(point.0);
            x_max = x_max.max(point.0);
            y_min = y_min.min(point.1);
            y_max = y_max.max(point.1);

            points.push(point);
            // points.push((x, y));
            step += 1;

            terminal.draw(|f| {
                let size = f.area();
                let chart_area = Rect {
                    x: 0,
                    y: 0,
                    width: size.width,
                    height: size.height,
                };

                let dataset = Dataset::default()
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&points);

                let chart = Chart::new(vec![dataset])
                    // .block(Block::default().borders(Borders::ALL).title("Lorenz Attractor (X-Z)"))
                    .x_axis(
                        Axis::default() /*.title("X")*/
                            .bounds([x_min, x_max]),
                    )
                    .y_axis(
                        Axis::default() /*.title("Z")*/
                            .bounds([y_min, y_max]),
                    );

                f.render_widget(chart, chart_area);
            })?;

            // adjust delay for animation
            // thread::sleep(Duration::from_millis(1));
            if event::poll(Duration::from_millis(10))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                        break 'lorenz_attractor;
                    }
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
