use std::fmt;
use std::fmt::Display;

#[derive(PartialEq, Eq, Default, Clone)]
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
struct Token {
    value: String,
    kind: TokenKind,
    loc: Location,
}

#[derive(Default)]
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
                    Some(token) => " after token ".to_string() + &token.value 
                };
                write!(f, "Unable to lex token {}, at {}:{}", hint, cursor.loc.line, cursor.loc.col)
            }
        }
    }
}

fn lex(source: String) -> Result<Vec<Token>, LexError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut cursor = Cursor::default();
    let lexers: Vec<&dyn Fn(&String, &Cursor) -> Option<(Token, Cursor)>> = vec![
        &lexString,
        &lexIdentifier,
        &lexSymbol,
        &lexNumeric,
        &lexKeyword,
    ];

    while cursor.pointer < source.len() {
        let mut found_token = false;
        for lexer in &lexers {
            if let Some((token, new_cursor)) = lexer(&source, &cursor) {
                cursor = new_cursor;
                tokens.push(token);
                found_token = true;
                break;
            }
        }
        if !found_token {
            let t = match tokens.len() {
                0 => None, 
                n => Some(tokens[n-1].clone())
            };
            return Err(LexError::InvalidToken((t, cursor)));
        }
    }

    Ok(vec![])
}

fn lexNumeric(source: &String, init_cursor: &Cursor) -> Option<(Token, Cursor)> {

}

fn lexSymbol(source: &String, init_cursor: &Cursor) -> Option<(Token, Cursor)> {
    None
}

fn lexIdentifier(source: &String, init_cursor: &Cursor) -> Option<(Token, Cursor)> {
    None
}

fn lexString(source: &String, init_cursor: &Cursor) -> Option<(Token, Cursor)> {
    None
}

fn lexKeyword(source: &String, init_cursor: &Cursor) -> Option<(Token, Cursor)> {
    None
}
