use std::collections::HashMap;

use crate::ast::{CreateStatement, InsertStatement, SelectItem, SelectStatement};
use crate::database::{CellType, Column, Database, QueryResult};
use crate::lexer::Token;

pub struct Memory {
    tables: HashMap<String, Table>,
}

impl Database for Memory {
    fn insert(&mut self, insert_statement: InsertStatement) -> Result<()> {
        let table = self.get_table_mut(&insert_statement.table.value)?;
        let row = Vec::<CellValue>::new();

        for value_token in insert_statement.values {
            row.push(CellValue::from(value_token));
        }

        table.insert_row(row);

        Ok(())
    }

    fn select(&self, select_statement: SelectStatement) -> Result<QueryResult> {
        let table = self.get_table(&select_statement.table_name.value)?;

        let rows = Vec::<Vec<Cell>>::new();
        for (row_index, _) in table.rows.iter().enumerate() {
            let row_cells = Vec::<Cell>::new();
            for select_item in select_statement.items {
                let cell = table.get_cell(&select_item, row_index)?;
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
        let table_name = create_statement.name.value;
        let table_search_result = self.get_table(&table_name);
        if table_search_result.is_ok() {
            return Err(MemoryError::TableAlreadyExists(table_name);
        }

        let table = Table::from_create_statement(create_statement)?;

        self.insert_table(&table_name, table);

        Ok(())
    }
}

#[derive(Default)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    rows: Vec<Vec<CellValue>>,
}

impl Table {
    pub fn insert_row(&mut self, row: Vec<CellValue>) {
        self.rows.push(row)
    }

    pub fn get_cell(&self, select_item: &SelectItem, row_index: usize) -> Result<Cell> {
        let (column, col_index) = self.get_column_from_select_item(select_item)?;
         Ok(Cell {
                value: self.rows[row_index][col_index],
                cell_type: column.column_type,
                column_name: select_item
                    .as_name
                    .map_or(column.name, |name_token| name_token.value),
        })
    }

    // returns columns and column index
    pub fn get_column_from_select_item(&self, select_item: &SelectItem) -> Result<(Column, usize)> {
        for (col_index, column) in self.columns.iter().enumerate() {
            if column.name == select_item.name.value {
                return Ok((Column{
                    name: select_item
                    .as_name
                    .map_or(column.name, |name_token| name_token.value),
                    column_type: column.column_type
                }, col_index));
            }
        }
        Err(MemoryError::ColumnNotFound(select_item.name.value))
    }
}

impl Table {
    pub fn from_create_statement(create_statement: CreateStatement) -> Result<Self> {
        let columns = vec![];
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

#[derive(Default)]
pub struct Cell {
    value: CellValue,
    column_name: String,
    cell_type: CellType,
}

#[derive(Default)]
pub struct CellValue;

impl From<Token> for CellValue {
    fn from(token: Token) -> Self {}
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
        self.tables.insert(table_name, table);
    }
}

pub type Result<T> = std::result::Result<T, MemoryError>;

pub enum MemoryError {
    // the String is the table name
    TableNotFound(String),
    ColumnNotFound(String),
    TableAlreadyExists(String),
    InvalidType(String)
}
