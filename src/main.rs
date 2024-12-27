use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::read,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;

fn main() -> anyhow::Result<()> {
    let mut stdout: std::io::Stdout = stdout();
    // let mut cx = 0;
    // let mut cy = 0;
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    let (cols, rows) = terminal::size()?;
    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(0..cols);
    let mut y = rng.gen_range(0..rows);
    loop {
        // stdout.queue(cursor::MoveTo(cx, cy))?;
        stdout.queue(cursor::MoveTo(x, y))?;
        stdout.queue(crossterm::style::Print(" "))?;
        x = rng.gen_range(0..cols);
        y = rng.gen_range(0..rows);
        stdout.queue(cursor::Hide)?;
        stdout
            .execute(cursor::MoveTo(x, y))?
            .execute(crossterm::style::SetForegroundColor(
                crossterm::style::Color::Green,
            ))?
            .execute(crossterm::style::Print("*"))?;
        stdout.flush()?;

        match read()? {
            crossterm::event::Event::Key(event) => match event.code {
                crossterm::event::KeyCode::Char('q') => break,

                _ => {}
            },
            _ => {}
        }
    }
    stdout.queue(cursor::Show)?;

    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
