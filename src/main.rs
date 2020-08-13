mod ast;
mod cursor;
mod lex_error;
mod lexer;
use crate::ast::Statement;
use crate::lex_error::LexError;
use crate::lexer::lex;

fn main() {
    // let source = "select name from users where user_id = 1;
    // and email = 'something@gmail.com'";

    // let tokens = lex(source)?;

    // for token in tokens {
    //     println!("{:?}", token);
    // }

    let source = "insert into hello values (one, two, three )";

    let tokens = lex(source).unwrap();

    let stmt = ast::InsertStatement::from_tokens(&tokens).unwrap();

    println!("{:#?}", stmt.unwrap());
}
