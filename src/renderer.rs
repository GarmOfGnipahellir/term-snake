use cgmath::Vector2;
use crossterm::{
    cursor, execute, queue,
    style::{self, StyledContent},
    terminal, Result,
};
use std::io::{stdout, Write};

pub trait Renderable {
    fn position(&self) -> Vector2<u16>;
    fn character(&self) -> StyledContent<&str>;
}

impl dyn Renderable {
    pub fn collides(&self, other: &Box<dyn Renderable>) -> bool {
        self.position() == other.position()
    }

    pub fn collides_any(&self, renderables: &Vec<Box<dyn Renderable>>) -> bool {
        for other in renderables {
            if self.collides(other) {
                return true;
            }
        }
        false
    }
}

pub struct Renderer;

impl Renderer {
    pub fn new() -> Renderer {
        Renderer
    }

    pub fn render(&self, renderables: &Vec<Box<dyn Renderable>>) -> Result<()> {
        let mut stdout = stdout();

        execute!(
            stdout,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All)
        )?;

        for r in renderables {
            queue!(
                stdout,
                cursor::MoveTo(r.position().x * 2, r.position().y),
                style::PrintStyledContent(r.character())
            )?;
        }

        stdout.flush()?;

        Ok(())
    }
}
