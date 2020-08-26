use crate::lexer::{KeywordType, SymbolType, Token, TokenKind};
use std::borrow::BorrowMut;
use std::ops::Deref;

#[derive(Debug)]
enum ErrorKind {
    MissingIntoKeyword,
    MissingTableName,
    MissingValuesKeyword,
    MissingLeftParen,
    MissingRightParens,
    UnexpectedAsKeyword,
    ExpectedNameAfterAs,
    ExpectedComma,
    ExpectedTableNameAfterCreate,
    ExpectedColumnType,
    ExpectedCommaOrRightParen,
    InvalidType,
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
    pub table: Token,
    pub values: Vec<Token>,
}

#[derive(Debug)]
pub struct CreateStatement {
    pub name: Token,
    pub cols: Vec<Column>,
}

#[derive(Debug)]
pub struct Column {
    pub name: Token,
    pub data_type: Token,
    pub is_primary_key: bool,
}

#[derive(Debug)]
pub struct SelectStatement {
    pub table_name: Token,
    pub items: Vec<SelectItem>,
}

pub trait Parsable: Sized {
    fn from_tokens(tokens: &[Token]) -> Result<Option<Self>, ParseError>;
}

fn expect_token(
    token: Option<&Token>,
    expected_token_kind: TokenKind,
    error_kind: ErrorKind,
) -> Result<(), ParseError> {
    if token.is_none() {
        return Err(ParseError {
            token: Token::empty_token(),
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

impl Parsable for InsertStatement {
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
                token: Token::empty_token(),
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
                    token: Token::empty_token(),
                    error_kind: ErrorKind::MissingLeftParen,
                });
            }
            let token = token.unwrap();

            match token.kind {
                TokenKind::Identifier | TokenKind::Numeric => values.push(token.clone()),
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

impl Parsable for SelectStatement {
    // select
    // [...$value [ as $name ] ]
    // from
    // $table_name
    //
    fn from_tokens(tokens: &[Token]) -> Result<Option<Self>, ParseError> {
        let mut tokens = tokens.iter();

        if tokens.next().unwrap().kind != TokenKind::Keyword(KeywordType::Select) {
            return Ok(None);
        }

        let mut select_items: Vec<SelectItem> = vec![];
        let mut is_comma = false;

        loop {
            if let Some(token) = tokens.next() {
                match token.kind {
                    TokenKind::Identifier | TokenKind::Numeric => {
                        if !select_items.is_empty() && !is_comma {
                            return Err(ParseError {
                                token: token.clone(),
                                error_kind: ErrorKind::ExpectedComma,
                            });
                        }
                        select_items.push(SelectItem {
                            name: token.clone(),
                            as_name: None,
                        });
                        is_comma = false;
                    }
                    TokenKind::Keyword(KeywordType::As) => {
                        if let Some(last_item) = select_items.last_mut() {
                            if let Some(as_name_token) = tokens.next() {
                                last_item.as_name = Some(as_name_token.clone());
                            } else {
                                return Err(ParseError {
                                    token: Token::empty_token(),
                                    error_kind: ErrorKind::ExpectedNameAfterAs,
                                });
                            }
                        } else {
                            return Err(ParseError {
                                token: token.clone(),
                                error_kind: ErrorKind::UnexpectedAsKeyword,
                            });
                        }
                    }
                    TokenKind::Symbol(SymbolType::Comma) => is_comma = true,
                    _ => {
                        expect_token(
                            Some(token),
                            TokenKind::Keyword(KeywordType::From),
                            ErrorKind::MissingTableName,
                        )?;
                        break;
                    }
                }
            } else {
                return Err(ParseError {
                    token: Token::empty_token(),
                    error_kind: ErrorKind::MissingTableName,
                });
            }
        }

        if let Some(table_name_token) = tokens.next() {
            Ok(Some(SelectStatement {
                items: select_items,
                table_name: table_name_token.clone(),
            }))
        } else {
            Err(ParseError {
                token: Token::empty_token(),
                error_kind: ErrorKind::MissingTableName,
            })
        }
    }
}

impl Parsable for CreateStatement {
    // create
    // table $table_name
    // (
    //  [$name $type]
    // )
    fn from_tokens(tokens: &[Token]) -> Result<Option<Self>, ParseError> {
        let mut tokens = tokens.iter();

        if tokens.next().unwrap().kind != TokenKind::Keyword(KeywordType::Create) {
            return Ok(None);
        }

        expect_token(
            tokens.next(),
            TokenKind::Keyword(KeywordType::Table),
            ErrorKind::ExpectedTableNameAfterCreate,
        )?;

        let table_name_token = tokens.next();
        if table_name_token.is_none() {
            return Err(ParseError {
                error_kind: ErrorKind::MissingTableName,
                token: Token::empty_token(),
            });
        }
        let table_name_token = table_name_token.unwrap();

        expect_token(
            tokens.next(),
            TokenKind::Symbol(SymbolType::LeftParen),
            ErrorKind::MissingLeftParen,
        )?;

        let mut cols = Vec::<Column>::new();

        loop {
            let col_name = tokens.next();
            if col_name.is_none() {
                return Err(ParseError {
                    token: Token::empty_token(),
                    error_kind: ErrorKind::MissingRightParens,
                });
            }
            let col_name = col_name.unwrap();
            let col_type = tokens.next();
            if col_type.is_none() {
                return Err(ParseError {
                    token: Token::empty_token(),
                    error_kind: ErrorKind::ExpectedColumnType,
                });
            }
            let col_type = col_type.unwrap();
            if col_type.kind != TokenKind::Keyword(KeywordType::Int)
                && col_type.kind != TokenKind::Keyword(KeywordType::Text)
            {
                return Err(ParseError {
                    token: col_type.clone(),
                    error_kind: ErrorKind::InvalidType,
                });
            }
            cols.push(Column {
                name: col_name.clone(),
                data_type: col_type.clone(),
                is_primary_key: false,
            });
            let comma_or_right_paren = tokens.next();
            if comma_or_right_paren.is_none() {
                return Err(ParseError {
                    error_kind: ErrorKind::MissingRightParens,
                    token: Token::empty_token(),
                });
            }
            let comma_or_right_paren = comma_or_right_paren.unwrap();
            match comma_or_right_paren.kind {
                TokenKind::Symbol(SymbolType::RightParen) => break,
                TokenKind::Symbol(SymbolType::Comma) => continue,
                _ => {
                    return Err(ParseError {
                        token: comma_or_right_paren.clone(),
                        error_kind: ErrorKind::ExpectedCommaOrRightParen,
                    })
                }
            }
        }

        Ok(Some(CreateStatement {
            cols,
            name: table_name_token.clone(),
        }))
    }
}

#[derive(Debug, Clone)]
pub struct SelectItem {
    pub name: Token,
    pub as_name: Option<Token>,
}
