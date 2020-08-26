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

fn main() {
    let create_statement_source = "create table mytable (id text , name text)";
    let create_tokens = lex(create_statement_source).unwrap();
    let create_statement = ast::CreateStatement::from_tokens(&create_tokens).unwrap().unwrap();

    let insert_statement_source = "insert into mytable values (\'one\' , \'two\' )";
    let insert_tokens = lex(insert_statement_source).unwrap();
    insert_tokens.iter().for_each(|t| println!("{:?}", t));
    let insert_statement = ast::InsertStatement::from_tokens(&insert_tokens)
        .unwrap()
        .unwrap();

    let select_statement_source = "select id from mytable";
    let select_tokens = lex(select_statement_source).unwrap();
    let select_statement = ast::SelectStatement::from_tokens(&select_tokens).unwrap().unwrap();

    let mut memory = Memory::default();
    memory.create_table(create_statement).unwrap();
    memory.insert(insert_statement).unwrap();
    let result = memory.select(select_statement).unwrap();
    println!("{:#?}", result);
}
