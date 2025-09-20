//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::{
    Context, GameError, GameResult,
    event::{self},
    glam::*,
    graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect},
    winit::event::MouseButton,
};
use jonsh_chess::board::Board;

const BOARD_SIZE: usize = 8;

struct MainState {
    board_mesh: Mesh,
    square_size: f32,
    board_size: f32,
    origin: Vec2,
    clicked_square: Option<(usize, usize)>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let w: f32 = 600.0;
        let h: f32 = 600.0;
        let pad = 20.0;
        let side = (w.min(h) - 2.0 * pad).floor();
        let origin = Vec2::new((w - side) * 0.5, (h - side) * 0.5);
        let square_size = side / BOARD_SIZE as f32;

        let board_mesh = Self::build_board_mesh(ctx, origin, square_size)?;

        Ok(Self {
            board_mesh,
            square_size,
            board_size: side,
            clicked_square: None,
            origin,
        })
    }

    fn build_board_mesh(
        ctx: &mut Context,
        origin: Vec2,
        square_size: f32,
    ) -> Result<Mesh, GameError> {
        let mut mb = MeshBuilder::new();

        let light = Color::from_rgb(240, 217, 181);
        let dark = Color::from_rgb(181, 136, 99);

        for r in 0..=7 {
            for c in 0..=7 {
                let x = origin.x + c as f32 * square_size;
                let y = origin.y + r as f32 * square_size;
                let bounds = Rect::new(x, y, square_size, square_size);
                let is_dark = (r + c) % 2 == 1;
                let color = if is_dark { dark } else { light };
                mb.rectangle(DrawMode::fill(), bounds, color)?;
            }
        }

        let data = mb.build();

        return Ok(Mesh::from_data(ctx, data));
    }

    fn px_to_square(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        let dx = x - self.origin.x;
        let dy = y - self.origin.y;

        if x < self.origin.x || y < self.origin.y {
            return None;
        };
        if dx > self.board_size || dy > self.board_size {
            return None;
        };

        let col = (dx / self.square_size).floor();
        let row = (dy / self.square_size).floor();

        Some((row as usize, col as usize))
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.board_mesh, DrawParam::default());

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::winit::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        if _button == MouseButton::Left {
            self.clicked_square = self.px_to_square(_x, _y);

            println!("{:?}", self.clicked_square)
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
