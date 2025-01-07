pub struct Config {
    pub word_size: u8,
    pub opcode_size: u8,
}

impl Config {
    pub fn new() -> Config {
        return Config {
            word_size: 8,
            opcode_size: 4
        }
    }
}