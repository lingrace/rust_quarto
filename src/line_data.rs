use crate::piece::Piece as Piece;


// LineData stores metadata about the current status of a winnable line.
pub struct LineData {
    cumulative_bit_and: u8, // defaults to 15, or 1111 in bits
    cumulative_bit_or: u8, // defaults to 0, or 0000 in bits
    number_of_pieces: i32,
}

impl Default for LineData {
    fn default() -> Self {
        Self {
            cumulative_bit_and: 0b1111, // 15 in decimal
            cumulative_bit_or: 0b0000, // 0 in decimal
            number_of_pieces: 0,
        }
    }
}

impl LineData {
    pub fn new() -> Self {
        LineData::default()
    }


    pub fn add_piece(&mut self, piece: Piece){
        self.number_of_pieces += 1;
        self.cumulative_bit_and &= piece.trait_bit_array(); 
        self.cumulative_bit_or |= piece.trait_bit_array();
    }
    pub fn is_win(&self) -> bool {
        // If the cumulative bit and is not 0, then there is a bit position where all the pieces have a 1
        // If the cumulative bit or is not 15, then there is a bit position where all the pieces have a 0
        return self.number_of_pieces == 4 && (self.cumulative_bit_and != 0 || self.cumulative_bit_or != 15)
        // TODO: turn hardcoded nums into constants
    }

}