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

pub fn make_move_string(fc: usize, fr: usize, tc: usize, tr: usize, white:bool) -> String {
    let files = ['A','B','C','D','E','F','G','H'];

    let from_file = files[fc];
    let from_rank = 8-fr;
    let to_file = files[tc];
    let to_rank = 8-tr;
    let piece = if white { "Q" } else { "q" };

    format!("{}{}{}{}{}", from_file, from_rank, to_file, to_rank, piece)
}

pub fn from_move_string(mv: String) -> (usize, usize, usize, usize, String) {
    let files = ['A','B','C','D','E','F','G','H'];

    let from_file = mv.chars().nth(0).expect("missing from file");
    let from_rank = mv.chars().nth(1).expect("missing from rank");
    let to_file   = mv.chars().nth(2).expect("missing to file");
    let to_rank   = mv.chars().nth(3).expect("missing to rank");
    let prom_piece = mv.chars().nth(4).expect("missing prom piece");

    let ff_idx = files.iter()
                      .position(|&c| c == from_file.to_ascii_uppercase())
                      .expect("invalid from file");

    let tf_idx = files.iter()
                      .position(|&c| c == to_file.to_ascii_uppercase())
                      .expect("invalid to file");

    let fr_idx = 8-from_rank.to_digit(10).expect("invalid from rank") as usize;
    let tr_idx = 8-to_rank.to_digit(10).expect("invalid to rank") as usize;

    (ff_idx, fr_idx, tf_idx, tr_idx, prom_piece.to_string())
}