## RUQL is an SQL database from scratch written in Rust

### Important Files:

```
├── src
   ├── ast.rs                    # Parser & AST definition
   ├── database.rs               # Database and query result definitions
   ├── table.rs                  # Implementation of the database and functions that run queries
   ├── lexer.rs                  # the lexer
   └── main.rs                   # the entry point and the repl
```

### Run It

*In order to run this, you need to have rust [installed](https://www.rust-lang.org/tools/install)*

Clone the project and run 'Cargo run' in the command line

### Example

```

#> create table mytable (name text , id int )
query executed
#> insert into mytable values ('ruql', 1 )
query executed
#> select name, id from mytable
name | id | 
------------------
'ruql' | 1 | 
------------------
END
```
