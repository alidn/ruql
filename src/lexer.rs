use std::fmt;
use std::fmt::Display;

#[derive(PartialEq, Eq, Default, Clone, Copy)]
struct Location {
    line: usize,
    col: usize,
}

#[derive(PartialEq, Eq, Clone)]
enum Keyword {
    SelectKeyword,
    FromKeyword,
    AsKeyword,
    TableKeyword,
    CreateKeyword,
    InsertKeyword,
    ValuesKeyword,
    IntKeyword,
    TextKeyword,
}

#[derive(PartialEq, Eq, Clone)]
enum Symbol {
    Semicolon,
    Asterisk,
    Comma,
    LeftParen,
    RightParen,
}

#[derive(PartialEq, Eq, Clone)]
enum TokenKind {
    Symbol(Symbol),
    Keyword(Keyword),
    Identifier,
    String,
    Numeric,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Token {
    value: String,
    kind: TokenKind,
    loc: Location,
}

#[derive(Default, Copy, Clone)]
struct Cursor {
    pointer: usize,
    loc: Location,
}

enum LexError {
    InvalidToken((Option<Token>, Cursor)),
}

impl Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::InvalidToken((prev_token, cursor)) => {
                let hint = match prev_token {
                    None => String::new(),
                    Some(token) => " after token ".to_string() + &token.value,
                };
                write!(
                    f,
                    "Unable to lex token {}, at {}:{}",
                    hint, cursor.loc.line, cursor.loc.col
                )
            }
        }
    }
}

fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut cursor = Cursor::default();
    let lexers: Vec<&dyn Fn(&str, &Cursor) -> Option<(Token, Cursor)>> = vec![
        &lexString,
        &lexIdentifier,
        &lexSymbol,
        &lexNumeric,
        &lexKeyword,
    ];

    while cursor.pointer < source.len() {
        let mut found_token = false;
        for lexer in &lexers {
            if let Some((token, new_cursor)) = lexer(&source[cursor.pointer..], &cursor) {
                cursor = new_cursor;
                tokens.push(token);
                found_token = true;
                break;
            }
        }
        if !found_token {
            let t = match tokens.len() {
                0 => None,
                n => Some(tokens[n - 1].clone()),
            };
            return Err(LexError::InvalidToken((t, cursor)));
        }
    }

    Ok(tokens)
}

fn lexNumeric(source: &str, init_cur: &Cursor) -> Option<(Token, Cursor)> {
    let mut cur = init_cur.clone();
    let mut found_period = false;
    let mut found_exp_mark = false;

    for c in source.chars() {
        cur.loc.col += 1;
        cur.pointer += 1;

        let is_digit = c.is_ascii_digit();
        let is_period = c == '.';
        let is_exp_marker = c == 'e';

        if cur.pointer == init_cur.pointer {
            if !is_digit && !is_period {
                return None;
            }
            found_period = is_period;
            continue;
        }

        if is_period {
            if found_period {
                return None;
            }
            found_period = true;
            continue;
        }

        if is_exp_marker {
            if found_exp_mark {
                return None;
            }

            found_exp_mark = true;
            found_period = true;

            if cur.pointer - init_cur.pointer == source.len() {
                return None;
            }

            // TODO: handler +, -
        }

        if !is_digit {
            break;
        }
    }

    Some((
        Token {
            value: source[..cur.pointer].to_string(),
            loc: init_cur.loc,
            kind: TokenKind::Numeric,
        },
        cur,
    ))
}

fn lexSymbol(source: &str, init_cur: &Cursor) -> Option<(Token, Cursor)> {
    let mut cur = init_cur.clone();
    cur.loc.col += 1;
    cur.pointer += 1;
    if let Some(c) = source.chars().next() {
        match c {
            '\n' => {
                cur.loc.line += 1;
                cur.loc.col = 0;
                None
            }
            '\t' | ' ' => None,
            '(' => Some((
                Token {
                    value: c.to_string(),
                    loc: init_cur.loc,
                    kind: TokenKind::Symbol(Symbol::LeftParen),
                },
                cur,
            )),
            ';' => Some((
                Token {
                    value: c.to_string(),
                    loc: init_cur.loc,
                    kind: TokenKind::Symbol(Symbol::Semicolon),
                },
                cur,
            )),
            ')' => Some((
                Token {
                    value: c.to_string(),
                    loc: init_cur.loc,
                    kind: TokenKind::Symbol(Symbol::RightParen),
                },
                cur,
            )),
            '*' => Some((
                Token {
                    value: c.to_string(),
                    loc: init_cur.loc,
                    kind: TokenKind::Symbol(Symbol::Asterisk),
                },
                cur,
            )),
            '*' => Some((
                Token {
                    value: c.to_string(),
                    loc: init_cur.loc,
                    kind: TokenKind::Symbol(Symbol::Asterisk),
                },
                cur,
            )),
            ',' => Some((
                Token {
                    value: c.to_string(),
                    loc: init_cur.loc,
                    kind: TokenKind::Symbol(Symbol::Comma),
                },
                cur,
            )),
            _ => None,
        }
    } else {
        None
    }
}

fn lexIdentifier(source: &str, init_cur: &Cursor) -> Option<(Token, Cursor)> {
    None
}

fn lexString(source: &str, init_cur: &Cursor) -> Option<(Token, Cursor)> {
    None
}
fn lexKeyword(source: &str, init_cur: &Cursor) -> Option<(Token, Cursor)> {
    None
}
