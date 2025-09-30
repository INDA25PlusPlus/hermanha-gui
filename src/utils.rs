use jonsh_chess::board::Tile;
use jonsh_chess::pieces::{Piece, Color};

pub fn to_fen(tiles: [[Tile; 8]; 8]) -> String {
    let mut fen = String::new();
    let mut empty_squares: usize = 0;

    for row in tiles {
        for tile in row {
            if let Tile::Occupied(color, piece) = tile.to_owned() {
                if empty_squares > 0 {
                    fen.push_str(&empty_squares.to_string());
                    empty_squares = 0
                }
                let piece_char = match piece {
                    Piece::Bishop => "b",
                    Piece::GhostPawn => "p",
                    Piece::Pawn => "p",
                    Piece::King => "k",
                    Piece::Knight => "n",
                    Piece::Queen => "q",
                    Piece::Rook => "r",
                };

                let char = match color {
                    Color::Black => piece_char,
                    Color::White => &piece_char.to_uppercase()
                };

                fen.push_str(char);
                
            } else {
                empty_squares += 1
            }

        }
        if empty_squares > 0 {
            fen.push_str(&empty_squares.to_string());
            empty_squares = 0;
        }

        fen.push('/');
    }

    fen
}

// pub fn make_move_string(fc: usize, fr: usize, tc:usize, tr:usize) {
//     move_string = String::new()
// } 