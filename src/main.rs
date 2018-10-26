use std::io;

use Op::*;
use SyntaxTree::*;
use Token::*;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error reading line");

        lex(&input)
            .iter()
            .map(|a| println!("{:?}", a))
            .collect::<Vec<_>>();
    }
}

fn parse(code: &str) -> SyntaxTree {
    unimplemented!();
}

fn lex(code: &str) -> Vec<Token> {
    let mut res: Vec<Token> = vec![];
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '0'...'9' => {
                let start = i;
                while chars[i + 1].is_digit(10) {
                    i += 1;
                }
                res.push(Digit(
                    code[start..=i].parse().expect("Failed to parse digit"),
                ));
            }
            '+' | '-' | '*' | '/' => res.push(Op(chars[i])),
            '(' => res.push(ParenL),
            ')' => res.push(ParenR),
            _ => {}
        }

        i += 1;
    }

    res
}

enum SyntaxTree {
    Bin(Box<SyntaxTree>, Box<SyntaxTree>, Op),
    Num(usize),
}

enum Op {
    Plus,
    Minus,
    Mult,
    Divide,
}

#[derive(Debug)]
enum Token {
    ParenL,
    ParenR,
    Digit(usize),
    Op(char),
}
