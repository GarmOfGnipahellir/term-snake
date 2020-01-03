use crate::renderer::Renderable;
use cgmath::Vector2;
use crossterm::{
    style::{Colorize, StyledContent},
    terminal,
};
use std::convert::TryInto;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Into<Vector2<i16>> for Direction {
    fn into(self) -> Vector2<i16> {
        let mut result = Vector2::new(0, 0);
        match self {
            Direction::Left => result.x -= 1,
            Direction::Right => result.x += 1,
            Direction::Up => result.y -= 1,
            Direction::Down => result.y += 1,
        }
        result
    }
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Left => return Direction::Right,
            Direction::Right => return Direction::Left,
            Direction::Up => return Direction::Down,
            Direction::Down => return Direction::Up,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct Tail {
    position: Vector2<i16>,
    character: &'static str,
    direction: Direction,
}

impl Renderable for Tail {
    fn position(&self) -> Vector2<u16> {
        Vector2::new(
            self.position.x.try_into().unwrap(),
            self.position.y.try_into().unwrap(),
        )
    }

    fn character(&self) -> StyledContent<&str> {
        self.character.green()
    }
}

pub struct Snake {
    direction: Direction,
    pub tail: Vec<Tail>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            direction: Direction::Right,
            tail: vec![Tail {
                position: Vector2::new(0, 0),
                character: "█",
                direction: Direction::Right,
            }],
        }
    }

    pub fn turn(&mut self, new_dir: Direction) {
        if self.direction.opposite() == new_dir {
            return;
        }

        self.direction = new_dir;
    }

    pub fn slither(&mut self) -> Result<(), ()> {
        let old_tail = self.tail.clone();
        for (i, item) in self.tail.iter_mut().enumerate() {
            if i == 0 {
                item.position += self.direction.into();
            } else {
                item.position = old_tail.get(i - 1).unwrap().position;
            }

            let (wu, hu): (u16, u16) = terminal::size().unwrap();
            let w: i16 = wu.try_into().unwrap();
            let h: i16 = hu.try_into().unwrap();

            if item.position.x < 0 {
                item.position.x = w / 2;
            } else if item.position.x > w / 2 {
                item.position.x = 0;
            }

            if item.position.y < 0 {
                item.position.y = h;
            } else if item.position.y > h {
                item.position.y = 0;
            }
        }

        for item1 in self.tail.iter() {
            for item2 in self.tail.iter() {
                if item1 == item2 {
                    continue;
                }

                if item1.position == item2.position {
                    return Err(());
                }
            }
        }

        Ok(())
    }

    pub fn add_tail(&mut self) {
        let last = self.tail.last().unwrap();
        let tail = Tail {
            position: last.position,
            character: "▒",
            direction: last.direction,
        };
        self.tail.push(tail);
    }
}
