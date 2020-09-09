use std::collections::HashMap;

use crate::ast::{CreateStatement, InsertStatement, SelectItem, SelectStatement, FromSource};
use crate::database::{CellType, Column, Database, QueryResult};
use crate::lexer::Token;

#[derive(Default)]
pub struct Memory {
    tables: HashMap<String, Table>,
}

impl Database for Memory {
    fn run_query(&mut self, query: &str) -> std::result::Result<Option<QueryResult>, Box<dyn std::error::Error>> {
        let insert_stmt = InsertStatement::from_source(query)?;
        let select_stmt = SelectStatement::from_source(query)?;
        let create_stmt = CreateStatement::from_source(query)?;
        if let Some(stmt) = insert_stmt {
            self.insert(stmt)?;
            Ok(None)
        } else if let Some(stmt) = select_stmt {
            let result = self.select(stmt)?;
            Ok(Some(result))
        } else if let Some(stmt) = create_stmt {
            self.create_table(stmt)?;
            Ok(None)
        } else {
            Err(Box::new(MemoryError::QueryNotValid))
        }
    }

    fn insert(&mut self, insert_statement: InsertStatement) -> Result<()> {
        let table = self.get_table_mut(&insert_statement.table.value)?;
        let mut row = Vec::<CellValue>::new();

        for value_token in insert_statement.values {
            row.push(CellValue::from(value_token));
        }

        table.insert_row(row);

        Ok(())
    }

    fn select(&self, select_statement: SelectStatement) -> Result<QueryResult> {
        let table = self.get_table(&select_statement.table_name.value)?;

        let mut rows = Vec::<Vec<Cell>>::new();
        for (row_index, _) in table.rows.iter().enumerate() {
            let mut row_cells = Vec::<Cell>::new();
            for select_item in &select_statement.items {
                let cell = table.get_cell(select_item, row_index)?;
                row_cells.push(cell);
            }
            rows.push(row_cells);
        }

        let mut columns = Vec::<Column>::new();
        for select_item in select_statement.items {
            let (column, _) = table.get_column_from_select_item(&select_item)?;
            columns.push(column);
        }

        Ok(QueryResult {
            rows,
            columns
        })
    }

    fn create_table(&mut self, create_statement: CreateStatement) -> Result<()> {
        let table_name = create_statement.name.value.clone();
        let table_search_result = self.get_table(&table_name);
        if table_search_result.is_ok() {
            return Err(MemoryError::TableAlreadyExists(table_name.clone()));
        }

        let table = Table::from_create_statement(create_statement)?;

        self.insert_table(&table_name, table);

        Ok(())
    }
}

#[derive(Default)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<CellValue>>,
}

impl Table {
    pub fn insert_row(&mut self, row: Vec<CellValue>) {
        self.rows.push(row)
    }

    pub fn get_cell(&self, select_item: &SelectItem, row_index: usize) -> Result<Cell> {
        let (column, col_index) = self.get_column_from_select_item(select_item)?;
         Ok(Cell {
                value: self.rows[row_index][col_index].clone(),
                cell_type: column.column_type,
                column_name: select_item.clone()
                    .as_name
                    .map_or(column.name, |name_token| name_token.value),
        })
    }

    // returns columns and column index
    pub fn get_column_from_select_item(&self, select_item: &SelectItem) -> Result<(Column, usize)> {
        for (col_index, column) in self.columns.iter().enumerate() {
            if column.name == select_item.name.value {
                return Ok((Column{
                    name: select_item.clone()
                    .as_name
                    .map_or(column.name.clone(), |name_token| name_token.value),
                    column_type: column.column_type.clone()
                }, col_index));
            }
        }
        Err(MemoryError::ColumnNotFound(select_item.name.value.clone()))
    }
}

impl Table {
    pub fn from_create_statement(create_statement: CreateStatement) -> Result<Self> {
        let mut columns = vec![];
        for column_token in create_statement.cols {
            columns.push(Column::parse_token(&column_token)?);
        }
        Ok(Table{
            name: create_statement.name.value,
            columns,
            rows: vec![]
        })
    }
}

#[derive(Default, Debug)]
pub struct Cell {
    pub value: CellValue,
    pub column_name: String,
    pub cell_type: CellType,
}

#[derive(Default, Clone, Debug)]
pub struct CellValue(String);

impl std::fmt::Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Token> for CellValue {
    fn from(token: Token) -> Self {
        CellValue(token.value)
    }
}

impl Memory {
    pub fn get_table(&self, table_name: &str) -> Result<&Table> {
        let table = self.tables.get(table_name);
        table.map_or(
            Err(MemoryError::TableNotFound(table_name.to_string())),
            |table| Ok(table),
        )
    }

    pub fn get_table_mut(&mut self, table_name: &str) -> Result<&mut Table> {
        let table = self.tables.get_mut(table_name);
        table.map_or(
            Err(MemoryError::TableNotFound(table_name.to_string())),
            |table| Ok(table),
        )
    }

    fn insert_table(&mut self, table_name: &str, table: Table) {
        self.tables.insert(table_name.to_string(), table);
    }
}

pub type Result<T> = std::result::Result<T, MemoryError>;

#[derive(Debug)]
pub enum MemoryError {
    // the String is the table name
    TableNotFound(String),
    ColumnNotFound(String),
    TableAlreadyExists(String),
    InvalidType(String),
    QueryNotValid
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::TableNotFound(name) => f.write_fmt(format_args!("table '{}' not found", name)),
            // TODO: add table name too
            MemoryError::ColumnNotFound(name) => f.write_fmt(format_args!("column '{}' not found", name)),
            MemoryError::TableAlreadyExists(name) => f.write_fmt(format_args!("table '{}' already exists", name)),
            MemoryError::InvalidType(type_name) => f.write_fmt(format_args!("type '{}' is not valid", type_name)),
            MemoryError::QueryNotValid => f.write_str("Query not valid")
        }
    }
}

impl std::error::Error for MemoryError {}