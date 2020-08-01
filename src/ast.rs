use crate::lexer::{Token, TokenKind, KeywordType};

struct Ast {
    pub statements: Statement
}

enum Statement {
    Select,
    Create(CreateStatement),
    Drop,
    Insert(InsertStatement)
}

struct InsertStatement {
    table: Token,
    values: Vec<Token>
}

struct CreateStatement {
    name: Token,
    cols: Vec<Column>
}

struct Column {
    name: Token,
    data_type: Token,
    is_primary_key: bool
}

struct SelectStatement {
    table_name: Token,
    items: Vec<SelectItem>
}

struct ParseError;

impl SelectStatement {
    /// insert
    /// into
    /// $table_name
    /// values
    /// (
    /// [...$value]
    /// )
    pub fn from_tokens(tokens: &[Token]) -> Result<Option<Self>, ParseError> {
        let mut token_index = 0;

        if tokens[token_index].kind != TokenKind::Keyword(KeywordType::Select) {
            return Ok(None)
        }



        Ok(None)
    }
}


struct SelectItem {
    name: Token,
    as_name: Option<Token>
}

