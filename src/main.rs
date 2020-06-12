mod error;
mod lexer;
use crate::error::Error;
use crate::lexer::lex;

fn main() -> Result<(), Error> {
    let source = "select name from users where user_id = 1;
    and email = 'something@gmail.com'";

    let tokens = lex(source)?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
