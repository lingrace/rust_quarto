pub struct Piece {
    // Each piece has four traits: height, color, shape (square or circle), and hollowness (hollow or filled).
    // Each of these traits has two options. We represent this information in the last 4 places a bit array.
    trait_bit_array: u8, 
}

impl Piece {
    pub fn new(trait_bit_array: u8) -> Self {
        assert!(trait_bit_array < 16, "trait_bit_array must be less than 16 (only 4 bits allowed)");
        Self { trait_bit_array }
    }

    pub fn trait_bit_array(&self) -> u8 {
        self.trait_bit_array
    }
}