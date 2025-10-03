use crate::piece::Piece as Piece;
use crate::line_data::LineData as LineData;

const BOARD_EDGE_SIZE: usize = 4;
const NUM_PIECES: usize = BOARD_EDGE_SIZE * BOARD_EDGE_SIZE;
const MAX_PLAYERS: usize = 2;

struct Player {
    name: String,
    // TODO: possibly will extend later to isHuman?
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {name: name.to_string()}
    }
}


// Trying out union style enums
enum GamePhase {
    GameInit,
    SelectPiece{current_player:Player, available_pieces: u16 },
    PlacePiece{current_player:Player, selected_piece: Piece},
    GameOver{winner: Option<Player>},
}

// TODO enum GameCommnands, e.g. help, instructions, pause, restart

pub struct GameState {
    board: [[Option<Piece>; BOARD_EDGE_SIZE]; BOARD_EDGE_SIZE],
    // There are 16 pieces in this game, each uniquely represented by a number between 0 and 15. A 16 bit bit array is sufficient to 
    available_pieces_array: [Option<Piece>; NUM_PIECES], // Contains an array of all the pieces. Pieces are moved to the board as they are placed.
    players: [Player; MAX_PLAYERS],
    game_phase: GamePhase,
    win_tracker_rows: [LineData;BOARD_EDGE_SIZE],
    win_tracker_columns: [LineData;BOARD_EDGE_SIZE],
    win_tracker_diagonals: [LineData;2],
    current_player_index: u8,
    selected_piece_index: Option<u8>,
}


impl GameState {
    pub fn new() -> Self {
        Self {
            board: std::array::from_fn(|_| std::array::from_fn(|_| None)),
            available_pieces_array: std::array::from_fn(|i| Some(Piece::new(i as u8))),
            players: std::array::from_fn(|i| Player::new(&format!("player_{}", i + 1))),
            game_phase: GamePhase::GameInit,
            win_tracker_rows: std::array::from_fn(|_| LineData::default()),
            win_tracker_columns: std::array::from_fn(|_| LineData::default()),
            win_tracker_diagonals: std::array::from_fn(|_| LineData::default()),
            current_player_index: 0,
            selected_piece_index: None,
        }
    }
    
    // select piece
    // place piece
    // check win status
    // increment phase
}

// // instructions
// // enter to move forwards

// psuedo code for game loop
// We are designing a game loop that can take input commands and increments the game phase
// Commands are likely 'help', 'display board', 'display available pieces', 'display game phase', 'debug', 'quit', 'restart'
// this will be an input based loop
// Each time there is input, first we check if one of the commands has been triggered
// then we match case based on phase, and type check
pub fn run(){
    loop {
        // handle the commands
        // match the game phase -> handle accordingly

        // as an example, if you are in the selecting piece phase, and there is no currently selected piece, prompt for a piece selection
        // if there is an input, type check it for piece selection. if yes, update the state and move the state to piece placement. the user then enters to trigger the loop again
        
    }
}