//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use std::collections::HashMap;

use ggez::{
    Context, GameError, GameResult,
    event::{self},
    glam::*,
    graphics::{self, Canvas, Color, DrawMode, DrawParam, Image, Mesh, MeshBuilder, Rect},
    winit::event::MouseButton,
};
use jonsh_chess::board::{Board, Tile};
use jonsh_chess::pieces;

const BOARD_SIZE: usize = 8;

struct MainState {
    board: Board,
    board_mesh: Mesh,
    square_size: f32,
    board_size: f32,
    origin: Vec2,
    clicked_square: Option<(usize, usize)>,
    pieces: HashMap<&'static str, Image>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let board = Board::new();
        let w: f32 = 600.0;
        let h: f32 = 600.0;
        let pad = 20.0;
        let side = (w.min(h) - 2.0 * pad).floor();
        let origin = Vec2::new((w - side) * 0.5, (h - side) * 0.5);
        let square_size = side / BOARD_SIZE as f32;

        let board_mesh = Self::build_board_mesh(ctx, origin, square_size)?;

        // load piece images once
        let mut pieces = std::collections::HashMap::new();
        pieces.insert("wp", Image::from_path(ctx, "/pieces/white-pawn.png")?);
        pieces.insert("bp", Image::from_path(ctx, "/pieces/black-pawn.png")?);
        pieces.insert("wb", Image::from_path(ctx, "/pieces/white-bishop.png")?);
        pieces.insert("bb", Image::from_path(ctx, "/pieces/black-bishop.png")?);
        pieces.insert("wq", Image::from_path(ctx, "/pieces/white-queen.png")?);
        pieces.insert("bq", Image::from_path(ctx, "/pieces/black-queen.png")?);
        pieces.insert("wk", Image::from_path(ctx, "/pieces/white-knight.png")?);
        pieces.insert("bk", Image::from_path(ctx, "/pieces/black-knight.png")?);
        pieces.insert("wr", Image::from_path(ctx, "/pieces/white-rook.png")?);
        pieces.insert("br", Image::from_path(ctx, "/pieces/black-rook.png")?);
        pieces.insert("wK", Image::from_path(ctx, "/pieces/white-king.png")?);
        pieces.insert("bK", Image::from_path(ctx, "/pieces/black-king.png")?);

        Ok(Self {
            board,
            board_mesh,
            square_size,
            board_size: side,
            clicked_square: None,
            origin,
            pieces,
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

    fn place_pieces(&self, canvas: &mut Canvas) -> Result<(), GameError> {
        for (row_i, row) in self.board.tiles.iter().enumerate() {
            for (col_i, tile) in row.iter().enumerate() {
                let Tile::Occupied(color, piece) = tile else {
                    continue;
                };

                let color_string = match color {
                    pieces::Color::Black => "b",
                    pieces::Color::White => "w",
                };
                let piece_string = match piece {
                    pieces::Piece::Bishop => "b",
                    pieces::Piece::GhostPawn => "p",
                    pieces::Piece::Pawn => "p",
                    pieces::Piece::King => "K",
                    pieces::Piece::Knight => "k",
                    pieces::Piece::Queen => "q",
                    pieces::Piece::Rook => "r",
                };

                let image_string = color_string.to_owned() + piece_string;
                let Some(image) = self.pieces.get(&*image_string) else {
                    return Err(GameError::GraphicsInitializationError);
                };
                let iw = image.width() as f32;
                let ih = image.height() as f32;
                let scale = Vec2::new(self.square_size / iw, self.square_size / ih);

                let y = row_i as f32 * self.square_size + self.origin.y;
                let x = col_i as f32 * self.square_size + self.origin.x;
                let dest = Vec2::new(x, y);

                canvas.draw(image, DrawParam::default().dest(dest).scale(scale));
            }
        }

        Ok(())
    }

    fn highlight_square(&self, ctx: &mut Context, row: usize, col: usize) -> Result<Mesh, GameError>{
        
        let x = self.origin.x + col as f32 * self.square_size;
        let y = self.origin.y + row as f32 * self.square_size;

        let mut builder = MeshBuilder::new();

        let bounds = Rect::new(x, y, self.square_size, self.square_size);
        let color = Color::from_rgba(0, 217, 0, 100);
        builder.rectangle(DrawMode::fill(), bounds, color)?;
        builder.build();

        let data = builder.build();

        return Ok(Mesh::from_data(ctx, data));
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

        if let Some((row, col)) = self.clicked_square {
            let highlight_mesh = self.highlight_square(ctx, row, col)?;
            canvas.draw(&highlight_mesh, DrawParam::default());
        }
        self.place_pieces(&mut canvas)?;

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::winit::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if _button == MouseButton::Left {
            if let Some((row, col)) = self.px_to_square(_x, _y) {
                match self.clicked_square {
                    None => {
                        self.clicked_square = Some((row, col));
                    }
                    Some((fr, fc)) => {
                        self.board.move_piece(fc, fr, col, row);
                        self.clicked_square = None;
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").add_resource_path("./assets"); // mount this dir
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
