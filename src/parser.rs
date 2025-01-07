use std::collections::HashMap;

use crate::Opcode;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Opcode(String),
    Variable(String),
    Operand(String),
    Number(isize)
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<Token>
}

pub type ParserResult = (Vec<Instruction>, HashMap<String, u8>);

pub fn lexer(input: &str, opcode_def: &HashMap<&str, Opcode>) -> Vec<Vec<Token>> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let mut tokens = Vec::new();
        let mut parts = line.split_whitespace();

        if let Some(first) = parts.next() {
            if opcode_def.contains_key(first) {
                tokens.push(Token::Opcode(first.to_string()));
            } else {
                tokens.push(Token::Variable(first.to_string()));
            }
        }

        for part in parts {
            if let Ok(num) = part.parse::<isize>() {
                tokens.push(Token::Number(num));
            } else {
                tokens.push(Token::Operand(part.to_string()));
            }
        }

        instructions.push(tokens);
    }

    instructions
}

pub fn parser(input: Vec<Vec<Token>>, opcode_def: &HashMap<&str, Opcode>) -> Result<ParserResult, String> {
    let mut instructions = Vec::new();
    let mut variables = HashMap::new();
    let mut line = 1;

    for expr in input {
        if expr.len() > 0 {
            match &expr[0] {
                Token::Opcode(opcode) => {
                    let opcode_info = opcode_def.get(opcode.as_str()).ok_or_else(|| {
                        format!("Unknown opcode: {}", opcode)
                    })?;
                    if expr.len() - 1 != opcode_info.operands {
                        return Err(format!(
                            "Line {}: Opcode {} expects {} operands, got {}.",
                            line, opcode, opcode_info.operands, expr.len() - 1
                        ));
                    }
                    instructions.push(Instruction {
                        opcode: opcode.clone(),
                        operands: expr[1..].to_vec(),
                    });
                },
                Token::Variable(var) => {
                    if expr.len() - 1 != 2 {
                        return Err(format!(
                            "Line {}: Variable declaration expects {} operands, got {}.",
                            line, 2, expr.len() - 1
                        ));
                    }
                    match &expr[2] {
                        Token::Number(num) => {
                            variables.insert(var.clone(), *num as u8);
                        },
                        _ => return Err(format!("Line {}: Unexpected operand after variable declaration", line))
                    }
                },
                _ => {
                    return Err(format!("Line {}: Unexpected token", line));
                }
            }
        }
        line += 1;
    }

    Ok((instructions, variables))
}
