use std::io;

use Op::*;
use SyntaxTree::*;
use Token::*;

macro_rules! create_binary_parser {
    ($name:ident $next:ident $($op:pat => $typ:ident),+) => {
        fn $name(toks: &Vec<Token>, i: &mut usize) -> Result<SyntaxTree, String> {
            let l = $next(toks, i)?;
            if i >= &mut toks.len() {
                return Ok(l);
            }
            $(
                if let $op = toks[*i] {
                    let o = $typ;
                    *i += 1;
                    let r = $name(&toks, i)?;
                    return Ok(Bin(Box::new(l), Box::new(r), o));
                }
            )+

            return Ok(l);
        }
    };
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error reading line");

        println!("{}", execute(parse(&input).unwrap()));
    }
}

fn execute(tree: SyntaxTree) -> isize {
    match tree {
        Num(n) => n,
        Bin(l, r, o) => {
            let l = execute(*l);
            let r = execute(*r);
            match o {
                Plus => l + r,
                Minus => l - r,
                Mult => l * r,
                Divide => l / r,
            }
        }
    }
}

fn parse(code: &str) -> Result<SyntaxTree, String> {
    let lexed = lex(&code);
    let mut i = 0;
    parse_bin(&lexed, &mut i)
}

create_binary_parser!(parse_bin parse_term Op('+') => Plus, Op('-') => Minus);
create_binary_parser!(parse_term parse_atom Op('*') => Mult, Op('/') => Divide);

fn parse_atom(toks: &Vec<Token>, i: &mut usize) -> Result<SyntaxTree, String> {
    match toks[*i] {
        ParenL => {
            *i += 1;
            let res = parse_bin(&toks, i)?;
            if *i >= toks.len() {
                return Err(String::from("parse_atom: expected ) but nothing"));
            }
            if let ParenR = toks[*i] {
            } else {
                return Err(String::from("parse_atom: mismatch )"));
            }
            *i += 1;
            Ok(res)
        }
        Digit(n) => {
            *i += 1;
            Ok(Num(n))
        }
        ref n => Err(String::from(format!(
            "parse_atom: not expecting token {:?}",
            n
        ))),
    }
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

#[derive(Debug)]
enum SyntaxTree {
    Bin(Box<SyntaxTree>, Box<SyntaxTree>, Op),
    Num(isize),
}

#[derive(Debug)]
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
    Digit(isize),
    Op(char),
}
