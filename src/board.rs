use crate::movedef::Movedef;
use crate::player::{Colour};
use std::{fmt};
use tabled::settings::{Style};
use tabled::tables::IterTable;

const BOARD_SIZE: usize = 8;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub loc: usize,
    pub king: bool,
    pub colour: Colour,
}

impl Piece {
    pub fn new(loc: usize, colour: Colour) -> Self {
        if loc >= BOARD_SIZE.pow(2) {
            panic!("coord larger that board size");
        } else {
            let me = Self {
                loc,
                king: false,
                colour,
            };
            me
        }
    }

    pub fn update_coord(&mut self, loc: usize) -> () {
        if loc >= BOARD_SIZE.pow(2) {
            panic!("coord {} larger that board size", loc);
        }
        self.loc = loc;
    }

    pub fn make_king(&mut self) -> () {
        self.king = true;
    }

    pub fn as_piece_string(&self) -> String {
        match self.colour {
            Colour::Black => {
                if self.king {
                    " 🔲 ".to_string()
                } else {
                    " ⬛️ ".to_string()
                }
            }
            Colour::White => {
                if self.king {
                    " 🔳 ".to_string()
                } else {
                    " ⬜️ ".to_string()
                }
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub struct Board {
    pub squares: [Option<Piece>; BOARD_SIZE.pow(2)],
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, square) in self.squares.iter().enumerate() {
            match square {
                Some(piece) => write!(f, "Square {}: {:?}", i, piece)?,
                None => write!(f, "Square {}: Empty", i)?,
            }
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        let mut b = [Option::None; BOARD_SIZE.pow(2)];
        for i in [1, 3, 5, 7, 8, 10, 12, 14, 17, 19, 21, 23] {
            //assign a mutable piece to the board
            b[i] = Option::from(Piece::new(i, Colour::White));
        }
        for i in [40, 42, 44, 46, 49, 51, 53, 55, 56, 58, 60, 62] {
            b[i] = Option::from(Piece::new(i, Colour::Black));
        }
        Self { squares: b }
    }

    pub fn get_row_col_from_index(loc: usize) -> (usize, usize) {
        if loc > BOARD_SIZE * BOARD_SIZE {
            panic!("not a valid board pos")
        }
        let row = loc / BOARD_SIZE;
        let col = loc % BOARD_SIZE;
        (row, col)
    }

    pub fn get_index_from_row_col(row: usize, col: usize) -> usize {
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            panic!("not a valid board pos")
        }
        let mut num = row * BOARD_SIZE;
        num += col;
        num
    }

    pub fn inside_board(x: usize, y: usize) -> bool {
        let x_b = x < BOARD_SIZE && x >= 0;
        let y_b = y < BOARD_SIZE && y >= 0;
        x_b && y_b
    }

    pub fn piece_get_crowned(loc: usize, colour: Colour) -> bool {
        let (row, _col) = Board::get_row_col_from_index(loc);
        match colour {
            Colour::Black => row == 0,
            Colour::White => row == BOARD_SIZE - 1,
        }
    }

    pub fn as_string(&self) -> String {
        let iterator = (0..BOARD_SIZE).map(|row| {
            (0..BOARD_SIZE).map(move |col| {
                let x = Board::get_index_from_row_col(row, col);
                let x = self.squares[x];
                match x {
                    None => {
                        format!("({}-{})\n", row, col)
                    }
                    Some(_) => {
                        format!("({}-{})\n{}", row, col, x.unwrap().as_piece_string())
                    }
                }
            })
        });

        let table = IterTable::new(iterator);
        let table = table.clone().with(Style::extended());

        table.to_string()
    }

    pub fn move_piece(&mut self, old_index: usize, new_index: usize) {
        if new_index >= BOARD_SIZE.pow(2) || self.squares[new_index] != None {
            panic!("Invalid move from ind: {}, coord: {:?} to ind: {}, coord:{:?}", old_index, Board::get_row_col_from_index(old_index), new_index, Board::get_row_col_from_index(new_index));
        }
        let piece = self.squares[old_index].unwrap();
        let mut piece_copy = piece.clone();
        piece_copy.update_coord(new_index);
        self.squares[old_index] = None;
        self.squares[new_index] = Some(piece_copy);
    }

    pub fn get_piece(&self, loc: usize) -> Option<&Piece> {
        self.squares[loc].as_ref()
    }

    pub fn get_all_colour_pieces(&self, colour: Colour) -> Vec<&Piece> {
        let mut pieces: Vec<&Piece> = Vec::new();
        for piece in self.squares.iter() {
            match piece {
                Some(piece) => {
                    if piece.colour == colour {
                        pieces.push(piece);
                    }
                }
                None => {}
            }
        }
        pieces
    }

    pub fn opposing_piece_between(
        &self,
        user_colour: Colour,
        row_s: usize,
        col_s: usize,
        row_e: usize,
        col_e: usize,
    ) -> Option<usize> {
        let row = (row_s + row_e) / 2;
        let col = (col_s + col_e) / 2;
        let index = Board::get_index_from_row_col(row, col);
        match self.squares[index] {
            Some(piece) => {
                if piece.colour != user_colour {
                    Some(index)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn ingest_movedef(&mut self, movedef: Movedef) -> () {
        self.move_piece(movedef.start, movedef.end);
        let mut king = false;
        match movedef.taken_piece {
            Some(x) => {
                king = self.squares[x].unwrap().king;
                self.squares[x] = None;
            }
            None => {}
        }
        let piece: &mut Piece = self.squares[movedef.end].as_mut().unwrap();
        if king || Board::piece_get_crowned(movedef.end, piece.colour)
        {
            piece.make_king();
        }
    }

    pub fn return_winner(&self) -> Option<Colour> {
        if self.get_all_colour_pieces(Colour::Black).is_empty() {Some(Colour::White)}
        else if self.get_all_colour_pieces(Colour::White).is_empty() {Some(Colour::Black)}
        else {None}
    }

    pub fn static_evaluation(&self, colour: Colour) -> i32 {
        let mut score = 0;
        for piece in self.get_all_colour_pieces(colour) {
            if piece.king {
                score += 5;
            } else {
                score += 3;
            }
        }
        let colour = colour.other();
        for piece in self.get_all_colour_pieces(colour) {
            if piece.king {
                score -= 5;
            } else {
                score -= 3;
            }
        }
        score
    }
}
