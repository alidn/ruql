use std::io::{self, BufRead, Write};

mod ast;
mod cursor;
mod database;
mod lex_error;
mod lexer;
mod table;

use crate::ast::Parsable;
use crate::database::Database;
use crate::lexer::lex;
use crate::table::Memory;

fn lex_tokens_example() {
    let source = "select name from users where user_id = 1;
    and email = 'something@gmail.com'";

    let tokens = lex(source).unwrap();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn insert_statement_example() {
    let source = "insert into hello values (\'one\', 2, three)";

    let tokens = lex(source).unwrap();
    tokens.iter().for_each(|t| println!("{:?}", t));

    let stmt = ast::InsertStatement::from_tokens(&tokens).unwrap();

    println!("{:#?}", stmt.unwrap());
}

fn select_statement_example() {
    let source = "select something as somethingelse from sometable";

    let tokens = lex(source).unwrap();
    tokens.iter().for_each(|t| println!("{:?}", t));

    let stmt = ast::SelectStatement::from_tokens(&tokens).unwrap();

    println!("{:#?}", stmt.unwrap());
}

fn create_statement_example() {
    let source = "create table my_table (id text , name text)";

    let tokens = lex(source).unwrap();
    tokens.iter().for_each(|t| println!("{:?}", t));

    let stmt = ast::CreateStatement::from_tokens(&tokens).unwrap();

    println!("{:#?}", stmt.unwrap());
}

fn run_repl() {
    let mut memory = Memory::default();
    println!();
    loop {
        print!("#> ");
        io::stdout().flush().unwrap();
        
        let stdin = std::io::stdin();
        let mut query = String::new();
        
        stdin.lock().read_line(&mut query).unwrap();

        let query_result = memory.run_query(&query);
        match query_result {
            Ok(res) => {
                match res {
                    Some(v) => println!("{}", v),
                    None => println!("query executed")
                }
            },
            Err(err) => println!("an error occurred: {}", err)
        }
    }
}

fn main() {
    run_repl();
}
