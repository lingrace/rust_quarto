use crate::piece::Piece as Piece;
use crate::line_data::LineData as LineData;

const BOARD_EDGE_SIZE: usize = 4;
const MAX_PLAYERS: usize = 2;

struct Player {
    name: String,
    // TODO: possibly will extend later to isHuman?
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
    available_pieces_bit_array: u16,
    players: [Player; MAX_PLAYERS],
    game_phase: GamePhase,
    win_tracker_rows: [LineData;BOARD_EDGE_SIZE],
    win_tracker_columns: [LineData;BOARD_EDGE_SIZE],
    win_tracker_diagonals: [LineData;2],
}


// impl GameState {
//     // init
//     // 
// }