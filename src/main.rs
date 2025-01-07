use std::collections::HashMap;
use std::env;
use binary::dump_binary;
use bitset::Bitset;
use glob::glob;
use std::io;

mod bitset;

mod config;
use config::Config;

mod binary;
use binary::generate_binary;

mod parser;
use parser::parser;
use parser::lexer;


#[derive(Debug)]
struct Opcode {
    binary: u8,
    operands: usize,
}

fn process_input(files: Vec<String>, opcode_def: &HashMap<&str, Opcode>, config: &Config) -> Result<Bitset, String> {
    let mut output = Bitset::new();

    for file in files {
        let input = std::fs::read_to_string(&file).map_err(|e| format!("Failed to read file '{}': {}", file, e))?;

        // Process the input with the parser and lexer
        match parser(lexer(&input, opcode_def), opcode_def) {
            Ok(res) => {
                generate_binary(&res, opcode_def, &config, &mut output)?;
            },
            Err(err) => {
                return Err(format!("Error parsing file '{}': {}", file, err));
            }
        }
    }

    Ok(output)
}

fn main() -> io::Result<()> {
    let config = Config::new();
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("sapxc: error: no input files");
        return Ok(());
    }

    let file_patterns = &args[1..];

    let opcode_def: HashMap<&str, Opcode> = HashMap::from([
        ("NOP", Opcode { binary: 0b0000, operands: 0 }),
        ("LDA", Opcode { binary: 0b0001, operands: 1 }),
        ("ADD", Opcode { binary: 0b0010, operands: 1 }),
        ("SUB", Opcode { binary: 0b0011, operands: 1 }),
        ("MUL", Opcode { binary: 0b0100, operands: 1 }),
        ("OUT", Opcode { binary: 0b0101, operands: 0 }),
        ("HLT", Opcode { binary: 0b0110, operands: 0 }),
        ("MI", Opcode { binary: 0b0111, operands: 1 }),
        ("RO", Opcode { binary: 0b1000, operands: 1 }),
        ("RI", Opcode { binary: 0b1001, operands: 1 }),
        ("IO", Opcode { binary: 0b1010, operands: 1 }),
        ("II", Opcode { binary: 0b1011, operands: 1 }),
        ("AO", Opcode { binary: 0b1100, operands: 1 }),
        ("AI", Opcode { binary: 0b1101, operands: 1 }),
        ("EO", Opcode { binary: 0b1110, operands: 1 }),
        ("SU", Opcode { binary: 0b1111, operands: 1 }),
    ]);

    let mut files = Vec::new();
    // Handle wildcards and collect files
    for pattern in file_patterns {
        let glob_pattern = format!("{}*", pattern);
        for entry in glob(&glob_pattern).unwrap().filter_map(Result::ok) {
            if entry.is_file() {
                files.push(entry.to_string_lossy().to_string());
            }
        }
    }

    // Process input files
    match process_input(files, &opcode_def, &config) {
        Ok(output) => {
            // If processing is successful, dump the binary
            match dump_binary(&output, "out.bin".to_string()) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("sapxc: error: {}", e);
                    Err(io::Error::new(io::ErrorKind::Other, e))
                }
            }
        },
        Err(e) => {
            eprintln!("sapxc: error: {}", e);
            Ok(())
        }
    }
}