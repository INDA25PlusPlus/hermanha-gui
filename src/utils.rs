use jonsh_chess::board::Tile;
use jonsh_chess::pieces::{Color, Piece};

pub const MSG_SIZE: usize = 128;

pub fn to_fen(tiles: [[Tile; 8]; 8]) -> String {
    let mut fen = String::new();
    let mut empty_squares: usize = 0;

    for (row_i, row) in tiles.iter().enumerate() {
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
                    Color::White => &piece_char.to_uppercase(),
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

        if row_i != 7 {
            fen.push('/');
        }
    }

    fen
}

pub fn make_move_string(fc: usize, fr: usize, tc: usize, tr: usize, white: bool) -> String {
    let files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    let from_file = files[fc];
    let from_rank = 8 - fr;
    let to_file = files[tc];
    let to_rank = 8 - tr;
    let piece = if white { "Q" } else { "q" };

    format!("{}{}{}{}{}", from_file, from_rank, to_file, to_rank, piece)
}

pub fn from_move_string(mv: &str) -> (usize, usize, usize, usize, String) {
    let files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    let from_file = mv.chars().nth(0).expect("missing from file");
    let from_rank = mv.chars().nth(1).expect("missing from rank");
    let to_file = mv.chars().nth(2).expect("missing to file");
    let to_rank = mv.chars().nth(3).expect("missing to rank");
    let prom_piece = mv.chars().nth(4).expect("missing prom piece");

    let fc = files
        .iter()
        .position(|&c| c == from_file.to_ascii_uppercase())
        .expect("invalid from file");

    let tc = files
        .iter()
        .position(|&c| c == to_file.to_ascii_uppercase())
        .expect("invalid to file");

    let fr = 8 - from_rank.to_digit(10).expect("invalid from rank") as usize;
    let tr = 8 - to_rank.to_digit(10).expect("invalid to rank") as usize;

    (fc, fr, tc, tr, prom_piece.to_string())
}

pub fn add_padding(msg: String) -> String {
    let len = msg.len();
    let diff = MSG_SIZE - len;
    if diff != 0 {
        return msg.to_owned() + ":" + &"0".repeat(diff - 1);
    } else {
        return msg.to_string();
    }
}

pub fn make_msg(quit: bool, mv: String, state: String, board: String) -> String {
    let msg_type = if quit { "ChessQUIT" } else { "ChessMOVE" };
    let msgs = vec![msg_type.to_string(), mv, state, board];

    let msg = msgs.join(":");
    add_padding(msg)
}
