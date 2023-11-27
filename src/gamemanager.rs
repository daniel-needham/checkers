use crate::board::Board;
use crate::movedef::Movedef;
use crate::player::Colour;
use crate::player::Player;
use rand::seq::IteratorRandom;
use rand::Rng;
use std::io;
use tabled::settings::Color;

enum GameState {
    Starting,
    PlayerTurn,
    AITurn,
    Ended,
}

pub struct GameManager {
    game_state: GameState,
    board: Option<Board>,
    player_colour: Option<Colour>,
    ai_colour: Option<Colour>,
}

impl<'a> GameManager {
    pub fn new() -> GameManager {
        Self {
            game_state: GameState::Starting,
            board: None,
            player_colour: None,
            ai_colour: None,
        }
    }

    pub fn set_board(&mut self, board: Board) {
        self.board = Some(board);
    }

    pub fn generate_legal_moves(&self, colour: Colour) -> Vec<Movedef> {
        let move_dir: i32 = match colour {
            Colour::White => 1,
            Colour::Black => -1,
        };
        let mut legal_moves: Vec<Movedef> = Vec::new();
        let board = self.board.as_ref().unwrap();
        let my_pieces = board.get_all_colour_pieces(colour);
        for piece in my_pieces {
            let coord = Board::get_row_col_from_index(piece.loc);
            let row = coord.0 as i32;
            let col = coord.1 as i32;
            //diagonal left
            let new_row = (row + move_dir) as usize;
            let new_col = (col - 1) as usize;
            if Board::inside_board(new_row as usize, new_col as usize) {
                // skip if outside board
                let new_index = Board::get_index_from_row_col(&new_row, &new_col);
                let new_piece = board.get_piece(new_index);
                match new_piece {
                    Some(_) => {}
                    None => {
                        //todo add  if at end of board
                        legal_moves.push(Movedef {
                            start: piece.loc,
                            end: new_index,
                            taken_piece: None,
                        })
                    }
                }
            }
            //diagonal right
            let new_row = (row + move_dir) as usize;
            let new_col = (col + 1) as usize;
            if Board::inside_board(new_row, new_col) {
                // skip if outside board
                let new_index = Board::get_index_from_row_col(&new_row, &new_col);
                let new_piece = board.get_piece(new_index);
                match new_piece {
                    Some(_) => {}
                    None => {
                        //todo add  if at end of board
                        legal_moves.push(Movedef {
                            start: piece.loc,
                            end: new_index,
                            taken_piece: None,
                        })
                    }
                }
            }
            //jump left
            let new_row = (row + (move_dir * 2)) as usize;
            let new_col = (col - 2) as usize;
            if Board::inside_board(new_row, new_col) {
                // skip if outside board
                match board.opposing_piece_between(
                    colour,
                    row as usize,
                    col as usize,
                    new_row,
                    new_col,
                ) {
                    Some(piece_to_take) => {
                        println!("piece to take: {:?}", piece_to_take);
                        let new_index = Board::get_index_from_row_col(&new_row, &new_col);
                        let new_piece = board.get_piece(new_index);
                        match new_piece {
                            Some(_) => {}
                            None => {
                                //todo add  if at end of board
                                legal_moves.push(Movedef {
                                    start: piece.loc,
                                    end: new_index,
                                    taken_piece: Option::from(piece_to_take),
                                })
                            }
                        }
                    }
                    None => {}
                }
            }
            //jump right
            let new_row = (row + (move_dir * 2)) as usize;
            let new_col = (col + 2) as usize;
            if Board::inside_board(new_row, new_col) {
                // skip if outside board
                match board.opposing_piece_between(
                    colour,
                    row as usize,
                    col as usize,
                    new_row,
                    new_col,
                ) {
                    Some(piece_to_take) => {
                        let new_index = Board::get_index_from_row_col(&new_row, &new_col);
                        let new_piece = board.get_piece(new_index);
                        match new_piece {
                            Some(_) => {}
                            None => {
                                //todo add  if at end of board
                                legal_moves.push(Movedef {
                                    start: piece.loc,
                                    end: new_index,
                                    taken_piece: Option::from(piece_to_take),
                                })
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        legal_moves
    }

    pub fn play_game(mut self) {
        match self.game_state {
            GameState::Starting => {
                // flip a coin to decide who goes first
                let mut rng = rand::thread_rng();
                let result = rng.gen_range(0..2);
                let mut input = String::new();
                while input != "H" && input != "T" {
                    println!("Type 'H' or 'T' to select heads or tails.");
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            // Print the user's input
                            println!("You entered: {}", input.trim());
                            input = input.trim().to_string();
                        }
                        Err(error) => {
                            eprintln!("Error reading input: {}", error);
                        }
                    }
                }
                // 0 is heads, 1 is tails
                if result == 0 {
                    println!("Heads");
                } else {
                    println!("Tails");
                }
                let won;
                if input == "H" && result == 0 || input == "T" && result == 1 {
                    println!("You win the coin toss! You start as white.");
                    won = true;
                } else {
                    println!("You lose the coin toss! You start as black.");
                    won = false;
                }

                let board = Board::new();
                self.set_board(board);

                // set player colour
                if won {
                    self.player_colour = Some(Colour::White);
                    self.ai_colour = Some(Colour::Black);
                    self.game_state = GameState::PlayerTurn;
                } else {
                    self.player_colour = Some(Colour::Black);
                    self.ai_colour = Some(Colour::White);
                    self.game_state = GameState::AITurn;
                }

                self.play_game();
            }
            GameState::PlayerTurn => {
                //Select square to move from
                let player_pieces = Board::get_all_colour_pieces(
                    self.board.as_ref().unwrap(),
                    self.player_colour.unwrap(),
                );
                let rand_piece = player_pieces.iter().choose(&mut rand::thread_rng());
                let (rand_row, rand_col) = Board::get_row_col_from_index(rand_piece.unwrap().loc);
                println!("{}", self.board.as_ref().unwrap().as_string());
                println!(
                    "Your turn! Select a piece to move. E.g '{}-{}'",
                    rand_row, rand_col
                );
                let legal_moves = self.generate_legal_moves(self.player_colour.unwrap());
                let selected_piece = loop {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            // Print the user's input
                            println!("You entered: {}", input.trim());
                            input = input.trim().to_string();
                            let split: Vec<&str> = input.split("-").collect();
                            if split.len() != 2 {
                                println!("Invalid input. Try again.");
                                continue;
                            }
                            let row = split[0].parse::<usize>();
                            let col = split[1].parse::<usize>();
                            if row.is_err() || col.is_err() {
                                println!("Invalid input. Try again.");
                                continue;
                            }
                            let row = row.unwrap();
                            let col = col.unwrap();
                            if !Board::inside_board(row, col) {
                                println!("That's not on the board. Try again.");
                                continue;
                            }
                            let index = Board::get_index_from_row_col(&row, &col);
                            let piece = self.board.as_ref().unwrap().get_piece(index);
                            match piece {
                                Some(piece) => {
                                    if piece.colour != self.player_colour.unwrap() {
                                        println!("That's not your piece. Try again.");
                                        continue;
                                    }
                                    if !legal_moves.iter().any(|&movedef| movedef.start == index) {
                                        println!("That piece can't move. Try again.");
                                        continue;
                                    }
                                    break piece;
                                }
                                None => {
                                    println!("There's no piece there. Try again.");
                                    continue;
                                }
                            }
                        }
                        Err(error) => {
                            eprintln!("Error reading input: {}", error);
                        }
                    }
                };
                //Select square to move to
                println!("Now select a square to move to");
                let selected_move = loop {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            // Print the user's input
                            println!("You entered: {}", input.trim());
                            input = input.trim().to_string();
                            let split: Vec<&str> = input.split("-").collect();
                            if split.len() != 2 {
                                println!("Invalid input. Try again.");
                                continue;
                            }
                            let row = split[0].parse::<usize>();
                            let col = split[1].parse::<usize>();
                            if row.is_err() || col.is_err() {
                                println!("Invalid input. Try again.");
                                continue;
                            }
                            let row = row.unwrap();
                            let col = col.unwrap();
                            if !Board::inside_board(row, col) {
                                println!("That's not on the board. Try again.");
                                continue;
                            }
                            if row == rand_row && col == rand_col {
                                println!("You can't move to the same square. Try again.");
                                continue;
                            }
                            let index = Board::get_index_from_row_col(&row, &col);
                            let piece = self.board.as_ref().unwrap().get_piece(index);
                            match piece {
                                Some(_) => {
                                    println!("There's already a piece there. Try again.");
                                    continue;
                                }
                                None => {
                                    let mut legal = false;
                                    let mut chosen_movedef: Option<Movedef> = None;
                                    for movedef in legal_moves.iter() {
                                        if movedef.start == selected_piece.loc
                                            && movedef.end == index
                                        {
                                            legal = true;
                                            chosen_movedef = Option::Some(*movedef);
                                            break;
                                        }
                                    }
                                    if !legal {
                                        println!("That's not a legal move. Try again.");
                                        continue;
                                    }
                                    break chosen_movedef;
                                }
                            }
                        }
                        Err(error) => {
                            eprintln!("Error reading input: {}", error);
                        }
                    }
                };
                println!("{:?}", selected_move.unwrap());
                self.board
                    .as_mut()
                    .unwrap()
                    .ingest_movedef(selected_move.unwrap()); //give move to board and update game state
                println!("{}", self.board.as_ref().unwrap().as_string()); //show move

                //testing only
                if self.player_colour == Some(Colour::White) {
                    self.player_colour = Some(Colour::Black);
                    self.ai_colour = Some(Colour::White);
                } else {
                    self.player_colour = Some(Colour::White);
                    self.ai_colour = Some(Colour::Black);
                }
                self.play_game();
                //testing only
            }
            GameState::AITurn => {
                println!("{}", self.board.as_ref().unwrap().as_string());
                println!("fuck you");
            }
            GameState::Ended => {
                println!()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::movedef::Movedef;
    use crate::player::Colour;

    #[test]
    fn move_piece() {
        let mut b = Board::new();
        let m = Movedef {
            start: 23,
            end: 30,
            taken_piece: None,
        };
        b.ingest_movedef(m);
        println!("{}", b.as_string());
        assert_eq!(b.squares[23], None);
        assert_eq!(b.squares[30].unwrap().colour, Colour::White);
    }

    #[test]
    fn take_piece() {
        let mut b = Board::new();
        let m = Movedef {
            start: 23,
            end: 30,
            taken_piece: None,
        };
        b.ingest_movedef(m);
        assert_eq!(b.squares[23], None);
        assert_eq!(b.squares[30].unwrap().colour, Colour::White);
        let m = Movedef {
            start: 44,
            end: 37,
            taken_piece: None,
        };
        b.ingest_movedef(m);
        assert_eq!(b.squares[30].unwrap().colour, Colour::White);
        let m = Movedef {
            start: 30,
            end: 44,
            taken_piece: Option::Some(37),
        };
        b.ingest_movedef(m);
        println!("{}", b.as_string());
        println!("{:?}", b);
        assert_eq!(b.squares[30], None);
        assert_eq!(b.squares[44].unwrap().colour, Colour::White);
        assert_eq!(b.squares[37], None);
    }
    #[test]
    fn piece_gets_crowned() {
        let mut b = Board::new();
        let m = Movedef {
            start: Board::get_index_from_row_col(&5, &4),
            end: Board::get_index_from_row_col(&4, &5),
            taken_piece: None,
        };
        b.ingest_movedef(m);
        let m = Movedef {
            start: Board::get_index_from_row_col(&4, &5),
            end: Board::get_index_from_row_col(&3, &6),
            taken_piece: None,
        };
        b.ingest_movedef(m);
        let m = Movedef {
            start: Board::get_index_from_row_col(&6, &3),
            end: Board::get_index_from_row_col(&5, &4),
            taken_piece: None,
        };
        b.ingest_movedef(m);
        let m = Movedef {
            start: Board::get_index_from_row_col(&5, &4),
            end: Board::get_index_from_row_col(&4, &3),
            taken_piece: None,
        };
        b.ingest_movedef(m);
        let m = Movedef {
            start: Board::get_index_from_row_col(&7, &2),
            end: Board::get_index_from_row_col(&6, &3),
            taken_piece: None,
        };
        b.ingest_movedef(m);
        let m = Movedef {
            start: Board::get_index_from_row_col(&2, &7),
            end: Board::get_index_from_row_col(&4, &5),
            taken_piece: Option::from(Board::get_index_from_row_col(&3, &6)),
        };
        b.ingest_movedef(m);
        let m = Movedef {
            start: Board::get_index_from_row_col(&4, &5),
            end: Board::get_index_from_row_col(&5, &4),
            taken_piece: None,
        };
        b.ingest_movedef(m);
        let m = Movedef {
            start: Board::get_index_from_row_col(&5, &4),
            end: Board::get_index_from_row_col(&7, &2),
            taken_piece: Option::from(Board::get_index_from_row_col(&6, &3)),
        };
        b.ingest_movedef(m);
        println!("{}", b.as_string());
        assert_eq!(b.squares[Board::get_index_from_row_col(&7, &2)].unwrap().king, true);
    }

}