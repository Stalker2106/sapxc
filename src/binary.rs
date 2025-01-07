use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

use crate::config::Config;
use crate::bitset::Bitset;
use crate::parser::{ParserResult, Token};
use crate::Opcode;

pub fn generate_binary(
    parser_result: &ParserResult,
    opcode_def: &HashMap<&str, Opcode>,
    config: &Config,
    output: &mut Bitset
) -> Result<(), String> {
    let (instructions, variables) = parser_result;
    let mut addressed_variables = HashMap::new();
    let mut next_free_address = (output.len() / config.word_size as usize) + (instructions.len() * (config.word_size as usize / 8));
    for instr in instructions {
        output.append(opcode_def.get(instr.opcode.as_str()).unwrap().binary as usize,config.opcode_size);
        for operand in &instr.operands {
            match operand {
                Token::Operand(var) => {
                    if !addressed_variables.contains_key(var) {
                        addressed_variables.insert(var.clone(), next_free_address);
                        next_free_address += config.word_size as usize / 8;
                    }
                    output.append(addressed_variables[var], config.word_size - config.opcode_size);
                },
                Token::Number(num) => {
                    output.append(*num as usize, config.word_size );
                }
                _ => {
                    return Err("Unexpected operand".to_string())
                }
            }
        }
        // pad missing bytes
        if instr.operands.len() == 0 {
            output.append(0, config.word_size - config.opcode_size);
        }
    }

    for (var, _addr) in addressed_variables.iter() {
        if !variables.contains_key(var) {
            return Err(format!("Undefined reference to variable {}", var));
        }
        output.append(variables[var] as usize, config.word_size);
    }

    return Ok(());
}

pub fn dump_binary(output: &Bitset, output_path: String) -> io::Result<()> {
    let mut outfile = File::create(output_path)?;

    // Write the bits as bytes into the file
    outfile.write_all(&output.get())?;

    Ok(())
}