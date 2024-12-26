use std::{
    error::Error,
    io::{self, Read},
};

enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    OpenLoop,
    CloseLoop,
    ProgramStart,
    ProgramEnd,
}

enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Vec<Instruction>),
    NoOp,
}

fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![Token::ProgramStart];
    for c in source.chars() {
        match c {
            '>' => tokens.push(Token::MoveRight),
            '<' => tokens.push(Token::MoveLeft),
            '+' => tokens.push(Token::Increment),
            '-' => tokens.push(Token::Decrement),
            '.' => tokens.push(Token::Output),
            ',' => tokens.push(Token::Input),
            '[' => tokens.push(Token::OpenLoop),
            ']' => tokens.push(Token::CloseLoop),
            _ => {}
        }
    }
    tokens.push(Token::ProgramEnd);
    tokens
}

fn parse(tokens: &[Token]) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();
    let mut stack_len = 0;
    let mut ret_ptr = 0;

    for (i, tok) in tokens
        .iter()
        .enumerate()
    {
        if stack_len > 0 && !matches!(tok, Token::OpenLoop | Token::CloseLoop) {
            continue;
        }

        match tok {
            Token::ProgramStart | Token::ProgramEnd => instructions.push(Instruction::NoOp),
            Token::MoveRight => instructions.push(Instruction::MoveRight),
            Token::MoveLeft => instructions.push(Instruction::MoveLeft),
            Token::Increment => instructions.push(Instruction::Increment),
            Token::Decrement => instructions.push(Instruction::Decrement),
            Token::Output => instructions.push(Instruction::Output),
            Token::Input => instructions.push(Instruction::Input),
            Token::OpenLoop => {
                if stack_len == 0 {
                    ret_ptr = i;
                }
                stack_len += 1;
            }
            Token::CloseLoop => {
                stack_len -= 1;
                match stack_len {
                    0 => {
                        let nested_instructions = parse(&tokens[ret_ptr + 1..i])?;
                        instructions.push(Instruction::Loop(nested_instructions));
                    }
                    x if x < 0 => return Err("Unexpected token: ']'".to_owned()),
                    _ => {}
                }
            }
        };
    }

    if stack_len != 0 {
        return Err("Unexpected token: '['".to_owned());
    }

    Ok(instructions)
}

fn interpret(instructions: &[Instruction], data: &mut [u8], dptr: &mut usize) {
    for instr in instructions {
        match instr {
            Instruction::MoveRight => *dptr += 1,
            Instruction::MoveLeft => *dptr -= 1,
            Instruction::Increment => data[*dptr] += 1,
            Instruction::Decrement => data[*dptr] -= 1,
            Instruction::Output => print!("{}", data[*dptr] as char),
            Instruction::Input => {
                let mut buf = [0; 1];
                io::stdin()
                    .read_exact(&mut buf)
                    .expect("Failed to readline");
                data[*dptr] = buf[0];
            }
            Instruction::Loop(in_loop_instr) => {
                while data[*dptr] != 0 {
                    interpret(in_loop_instr, data, dptr);
                }
            }
            Instruction::NoOp => {}
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let file_path = match args.nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: dotbf <FILE PATH>");
            std::process::exit(1);
        }
    };

    if !file_path.ends_with(".bf") {
        eprintln!("Invalid file type (.bf expected)");
        std::process::exit(1);
    }

    let contents = std::fs::read_to_string(file_path)?;
    let tokens = tokenize(&contents);
    let instructions = parse(&tokens)?;

    let mut data: [u8; 30_000] = [0; 30_000];
    let mut dptr: usize = 0;
    interpret(&instructions, &mut data, &mut dptr);

    Ok(())
}
