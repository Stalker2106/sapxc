#[derive(Debug)]
pub struct Bitset {
    buffer: Vec<u8>, // Holds the binary data
    cursor: u8,      // Tracks the current bit position (0-7) in the last byte
}

impl Bitset {
    /// Creates a new `Bitset`.
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            cursor: 0,
        }
    }

    /// Injects `bits` from the given `value` (of type `usize`) into the buffer.
    /// - `value` is the source `usize` containing bits to inject.
    /// - `bits` is the number of bits to take from `value` (1-64).
    pub fn append(&mut self, value: usize, bits: u8) {
        assert!(bits > 0 && bits <= 64, "Bits must be between 1 and 64");

        let mut remaining_bits = bits;
        let mut shifted_value = value; // Start with the full value as is

        while remaining_bits > 0 {
            if self.cursor == 0 {
                // Start a new byte if needed
                self.buffer.push(0);
            }

            let free_bits = 8 - self.cursor; // Available space in the current byte
            let write_bits = remaining_bits.min(free_bits); // How many bits we can write

            // Calculate the shift amount to move the correct bits to the correct position
            let shift = remaining_bits - write_bits;

            // Mask and shift correctly: only keep the lower 8 bits of the shifted value
            let value_to_write = ((shifted_value >> shift) & 0xFF) as u8;

            // Align the bits and write them to the byte
            self.buffer.last_mut().map(|byte| {
                // Ensure the left shift doesn't overflow the byte
                let shifted = value_to_write << (free_bits - write_bits);
                *byte |= shifted;
            });
            // Update cursor and remaining bits
            self.cursor = (self.cursor + write_bits) % 8;
            remaining_bits -= write_bits;

            // If cursor wraps around, move to the next byte
            if self.cursor == 0 {
                // Mask the already written bits
                shifted_value &= (1 << remaining_bits) - 1; // Keep only the remaining bits
            }
        }
    }

    /// Gets the current contents of the buffer.
    pub fn get(&self) -> &Vec<u8> {
        &self.buffer
    }

    /// Returns the number of bits used in the bitset.
    pub fn len(&self) -> usize {
        (self.buffer.len() * 8) - self.cursor as usize
    }
}