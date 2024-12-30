use std::io::{stdout, Write};

use beryllium::*;
// use crossterm::{
//     cursor,
//     event::read,
//     terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
//     ExecutableCommand, QueueableCommand,
// };
use rand::Rng;
fn main() {
    // Initialize SDL
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    // Set OpenGL context attributes
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    // #[cfg(target_os = "macos")]
    // {
    //     sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
    //         .unwrap();
    // }

    // Define window attributes
    let win_args = video::CreateWinArgs {
        title: "Snake Game",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    // Create the window and OpenGL context
    let _win = sdl
        .create_gl_window(win_args)
        .expect("Couldn't make a window and context");

    // Main event loop
    'main_loop: loop {
        // Handle events
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }

        // Here you can update the world state and render
    }
}

// fn main() -> anyhow::Result<()> {
//     let mut stdout: std::io::Stdout = stdout();
//     // let mut cx = 0;
//     // let mut cy = 0;
//     terminal::enable_raw_mode()?;
//     stdout.execute(EnterAlternateScreen)?;
//     stdout.execute(terminal::Clear(terminal::ClearType::All))?;
//     let (cols, rows) = terminal::size()?;
//     let mut rng = rand::thread_rng();
//     let mut x = rng.gen_range(0..cols);
//     let mut y = rng.gen_range(0..rows);
//     loop {
//         // stdout.queue(cursor::MoveTo(cx, cy))?;
//         stdout.queue(cursor::MoveTo(x, y))?;
//         stdout.queue(crossterm::style::Print(" "))?;
//         x = rng.gen_range(0..cols);
//         y = rng.gen_range(0..rows);
//         stdout.queue(cursor::Hide)?;
//         stdout
//             .execute(cursor::MoveTo(x, y))?
//             .execute(crossterm::style::SetForegroundColor(
//                 crossterm::style::Color::Green,
//             ))?
//             .execute(crossterm::style::Print("*"))?;
//         stdout.flush()?;

//         match read()? {
//             crossterm::event::Event::Key(event) => match event.code {
//                 crossterm::event::KeyCode::Char('q') => break,

//                 _ => {}
//             },
//             _ => {}
//         }
//     }
//     stdout.queue(cursor::Show)?;

//     stdout.execute(LeaveAlternateScreen)?;
//     terminal::disable_raw_mode()?;

//     Ok(())
// }
