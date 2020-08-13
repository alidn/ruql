use crate::lexer::{KeywordType, SymbolType, Token, TokenKind};

#[derive(Debug)]
enum ErrorKind {
    MissingIntoKeyword,
    MissingTableName,
    MissingValuesKeyword,
    MissingLeftParen,
    MissingRightParens,
}

#[derive(Debug)]
pub struct ParseError {
    token: Token,
    error_kind: ErrorKind,
}

struct Ast {
    pub statements: StatementType,
}

enum StatementType {
    Select,
    Create(CreateStatement),
    Drop,
    Insert(InsertStatement),
}

#[derive(Debug)]
pub struct InsertStatement {
    table: Token,
    values: Vec<Token>,
}

struct CreateStatement {
    name: Token,
    cols: Vec<Column>,
}

struct Column {
    name: Token,
    data_type: Token,
    is_primary_key: bool,
}

pub struct SelectStatement {
    table_name: Token,
    items: Vec<SelectItem>,
}

pub trait Statement: Sized {
    fn from_tokens(tokens: &[Token]) -> Result<Option<Self>, ParseError>;
}

fn expect_token(
    token: Option<&Token>,
    expected_token_kind: TokenKind,
    error_kind: ErrorKind,
) -> Result<(), ParseError> {
    if token.is_none() {
        return Err(ParseError {
            token: Token {
                value: String::new(),
                kind: TokenKind::String,
            },
            error_kind,
        });
    }

    let token = token.unwrap();

    if token.kind != expected_token_kind {
        Err(ParseError {
            token: token.clone(),
            error_kind,
        })
    } else {
        Ok(())
    }
}

impl Statement for InsertStatement {
    /// insert
    /// into
    /// $table_name
    /// values
    /// (
    /// [...$value]
    /// )
    fn from_tokens(tokens: &[Token]) -> Result<Option<Self>, ParseError> {
        let mut tokens = tokens.iter();

        if tokens.next().unwrap().kind != TokenKind::Keyword(KeywordType::Insert) {
            return Ok(None);
        }

        expect_token(
            tokens.next(),
            TokenKind::Keyword(KeywordType::Into),
            ErrorKind::MissingIntoKeyword,
        )?;

        let table_name_token = tokens.next();
        if table_name_token.is_none() {
            return Err(ParseError {
                token: Token {
                    value: "".to_string(),
                    kind: TokenKind::String,
                },
                error_kind: ErrorKind::MissingTableName,
            });
        }

        let table_name_token = table_name_token.unwrap();

        expect_token(
            tokens.next(),
            TokenKind::Keyword(KeywordType::Values),
            ErrorKind::MissingValuesKeyword,
        )?;

        expect_token(
            tokens.next(),
            TokenKind::Symbol(SymbolType::LeftParen),
            ErrorKind::MissingValuesKeyword,
        )?;

        let mut values = Vec::<Token>::new();

        loop {
            let token = tokens.next();
            if token.is_none() {
                return Err(ParseError {
                    token: Token {
                        value: String::new(),
                        kind: TokenKind::String,
                    },
                    error_kind: ErrorKind::MissingLeftParen,
                });
            }
            let token = token.unwrap();

            match token.kind {
                TokenKind::Identifier => values.push(token.clone()),
                TokenKind::Symbol(SymbolType::Comma) => continue,
                _ => {
                    expect_token(
                        Some(token),
                        TokenKind::Symbol(SymbolType::RightParen),
                        ErrorKind::MissingRightParens,
                    )?;
                    break;
                }
            }
        }

        Ok(Some(InsertStatement {
            table: table_name_token.clone(),
            values,
        }))
    }
}

struct SelectItem {
    name: Token,
    as_name: Option<Token>,
}
