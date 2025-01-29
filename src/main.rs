use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::Rng;
use std::{
    io,
    io::Write,
    time::{Duration, Instant},
};

struct Snake {
    body: Vec<(u16, u16)>,
    direction: Direction,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake: Snake,
    food: (u16, u16),
    score: u16,
    game_over: bool,
    width: u16,
    height: u16,
}

impl Snake {
    fn new(x: u16, y: u16) -> Self {
        Self {
            body: vec![(x, y), (x - 1, y), (x - 2, y)],
            direction: Direction::Right,
        }
    }

    fn move_forward(&mut self) {
        let head = self.body[0];
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1 - 1),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };
        self.body.insert(0, new_head);
        self.body.pop();
    }

    fn change_direction(&mut self, new_dir: Direction) {
        if !matches!(
            (new_dir, &self.direction),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        ) {
            self.direction = new_dir;
        }
    }
}

impl Game {
    fn new(width: u16, height: u16) -> Self {
        let mid_x = width / 2;
        let mid_y = height / 2;
        Self {
            snake: Snake::new(mid_x, mid_y),
            food: (0, 0),
            score: 0,
            game_over: false,
            width,
            height,
        }
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(1..self.width);
            let y = rng.gen_range(1..self.height);
            if !self.snake.body.contains(&(x, y)) {
                self.food = (x, y);
                break;
            }
        }
    }

    fn check_collision(&self) -> bool {
        let head = self.snake.body[0];

        // Check wall collision
        if head.0 == 0 || head.0 >= self.width || head.1 == 0 || head.1 >= self.height {
            return true;
        }

        // Check self collision
        self.snake.body.iter().skip(1).any(|&pos| pos == head)
    }

    fn update(&mut self) {
        self.snake.move_forward();

        if self.snake.body[0] == self.food {
            self.score += 1;
            self.snake.body.push(*self.snake.body.last().unwrap());
            self.spawn_food();
        }

        if self.check_collision() {
            self.game_over = true;
        }
    }

    fn draw(&self, prev_snake: &[(u16, u16)], prev_food: (u16, u16)) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Clear previous snake tail
        if let Some(&(x, y)) = prev_snake.last() {
            stdout.execute(crossterm::cursor::MoveTo(x, y))?;
            print!(" ");
        }

        // Draw new snake
        for &(x, y) in &self.snake.body {
            stdout.execute(crossterm::cursor::MoveTo(x, y))?;
            print!("■");
        }

        // Clear previous food
        stdout.execute(crossterm::cursor::MoveTo(prev_food.0, prev_food.1))?;
        print!(" ");

        // Draw new food
        stdout.execute(crossterm::cursor::MoveTo(self.food.0, self.food.1))?;
        print!("\x1b[31m●\x1b[0m");

        // Draw borders
        for x in 1..self.width {
            stdout.execute(crossterm::cursor::MoveTo(x, 0))?;
            print!("─");
            stdout.execute(crossterm::cursor::MoveTo(x, self.height))?;
            print!("─");
        }
        for y in 1..self.height {
            stdout.execute(crossterm::cursor::MoveTo(0, y))?;
            print!("│");
            stdout.execute(crossterm::cursor::MoveTo(self.width, y))?;
            print!("│");
        }

        // Draw corners
        stdout.execute(crossterm::cursor::MoveTo(0, 0))?;
        print!("┌");
        stdout.execute(crossterm::cursor::MoveTo(self.width, 0))?;
        print!("┐");
        stdout.execute(crossterm::cursor::MoveTo(0, self.height))?;
        print!("└");
        stdout.execute(crossterm::cursor::MoveTo(self.width, self.height))?;
        print!("┘");

        stdout.flush()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    // Terminal setup
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Game setup
    let width = 40; // Fixed width
    let height = 20; // Fixed height
    let mut game = Game::new(width, height);
    game.spawn_food();
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(75); // Faster speed

    let mut prev_snake = game.snake.body.clone();
    let mut prev_food = game.food;

    // Game loop
    while !game.game_over {
        // Input handling
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => game.snake.change_direction(Direction::Up),
                    KeyCode::Down => game.snake.change_direction(Direction::Down),
                    KeyCode::Left => game.snake.change_direction(Direction::Left),
                    KeyCode::Right => game.snake.change_direction(Direction::Right),
                    KeyCode::Char('q') | KeyCode::Esc => game.game_over = true,
                    _ => {}
                }
            }
        }

        // Update game state
        if last_update.elapsed() >= update_interval {
            game.update();
            last_update = Instant::now();
        }

        // Render
        game.draw(&prev_snake, prev_food)?;
        prev_snake = game.snake.body.clone();
        prev_food = game.food;
    }

    // Cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    println!("Game Over! Score: {}", game.score);
    Ok(())
}
