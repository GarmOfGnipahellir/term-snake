mod renderer;
mod snake;

use cgmath::Vector2;
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Colorize, StyledContent},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    Result,
};
use rand::{thread_rng, Rng};
use renderer::{Renderable, Renderer};
use snake::{Direction, Snake};
use std::io::{stdout, Write};
use std::time::Duration;

#[derive(Clone)]
struct Fruit {
    position: Vector2<u16>,
}

impl Renderable for Fruit {
    fn position(&self) -> Vector2<u16> {
        self.position.clone()
    }

    fn character(&self) -> StyledContent<&str> {
        "█".red()
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut rng = thread_rng();

    let mut score = 0;
    let mut snake = Snake::new();
    let mut fruit: Vec<Fruit> = Vec::new();
    let renderer = Renderer::new();

    let mut i = 0;
    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Left => snake.turn(Direction::Left),
                    KeyCode::Right => snake.turn(Direction::Right),
                    KeyCode::Up => snake.turn(Direction::Up),
                    KeyCode::Down => snake.turn(Direction::Down),
                    KeyCode::Char('t') => snake.add_tail(),
                    KeyCode::Esc => break,
                    _ => continue,
                }
            }
        }

        if let Err(()) = snake.slither() {
            break;
        }

        let mut boxed_fruit: Vec<Box<dyn Renderable>> = Vec::new();
        fruit.iter().for_each(|t| {
            boxed_fruit.push(Box::new(t.clone()));
        });

        let head: Box<dyn Renderable> = Box::new(snake.tail.first().unwrap().clone());
        if head.collides_any(&boxed_fruit) {
            if let Some(i) = fruit.iter().position(|f| f.position() == head.position()) {
                snake.add_tail();
                fruit.remove(i);
                score += 1;
            }
        }

        let mut objects: Vec<Box<dyn Renderable>> = Vec::new();
        snake.tail.iter().for_each(|t| {
            objects.push(Box::new(t.clone()));
        });
        fruit.iter().for_each(|t| {
            objects.push(Box::new(t.clone()));
        });
        renderer.render(&objects)?;

        let (w, h) = terminal::size().unwrap();

        if i % 10 == 0 {
            let position: Vector2<u16> = Vector2 {
                x: rng.gen_range(0, w / 2),
                y: rng.gen_range(0, h),
            };
            let f = Fruit { position: position };
            fruit.push(f);
        }

        i += 1;
    }

    disable_raw_mode()?;
    execute!(
        stdout(),
        cursor::Show,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
    )?;
    println!("term-snake finished with score: {}", score);

    Ok(())
}
