use crate::ast::{CreateStatement, InsertStatement, SelectStatement};
use crate::lexer::{KeywordType, Token, TokenKind};
use crate::table::{MemoryError, Result};
pub trait Database {
    fn create_table(&mut self, create_statement: CreateStatement) -> Result<()>;

    fn insert(&mut self, insert_statement: InsertStatement) -> Result<()>;

    fn select(&self, select_statement: SelectStatement) -> Result<QueryResult>;
}

#[derive(Debug)]
pub struct QueryResult {
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<crate::table::Cell>>,
}

#[derive(Debug, Clone)]
pub enum CellType {
    Int,
    Text,
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Text
    }
}

impl CellType {
    fn parse_token(token: &Token) -> Result<Self> {
        match token.kind {
            TokenKind::Keyword(KeywordType::Int) => Ok(CellType::Int),
            TokenKind::Keyword(KeywordType::Text) => Ok(CellType::Text),
            _ => Err(MemoryError::InvalidType(token.value.clone())),
        }
    }
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub column_type: CellType,
}

impl Column {
    pub fn parse_token(ast_column: &crate::ast::Column) -> Result<Self> {
        Ok(Column {
            name: ast_column.name.value.clone(),
            column_type: CellType::parse_token(&ast_column.data_type)?,
        })
    }
}