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
        if stack_len > 0 {
            match tok {
                Token::OpenLoop => stack_len += 1,
                Token::CloseLoop => {
                    stack_len -= 1;
                    if stack_len == 0 {
                        instructions.push(Instruction::Loop(parse(&tokens[ret_ptr + 1..i])?));
                    }
                }
                _ => {}
            }
            continue;
        }

        let instruction = match tok {
            Token::MoveRight => Instruction::MoveRight,
            Token::MoveLeft => Instruction::MoveLeft,
            Token::Increment => Instruction::Increment,
            Token::Decrement => Instruction::Decrement,
            Token::Output => Instruction::Output,
            Token::Input => Instruction::Input,
            Token::OpenLoop => {
                stack_len += 1;
                ret_ptr = i;
                Instruction::NoOp
            }
            Token::CloseLoop => {
                return Err("Unexpected token".to_owned());
            }
            Token::ProgramStart => Instruction::NoOp,
            Token::ProgramEnd => Instruction::NoOp,
        };
        instructions.push(instruction);
    }

    Ok(instructions)
}

fn interpret(
    instructions: &[Instruction],
    data: &mut [u8],
    dptr: &mut usize,
) -> Result<(), String> {
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
                    interpret(in_loop_instr, data, dptr)?;
                }
            }
            Instruction::NoOp => {}
        }
    }
    Ok(())
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
    interpret(&instructions, &mut data, &mut dptr)?;

    Ok(())
}
