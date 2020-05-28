use crate::lexer::TokenKind::Symbol;

#[derive(PartialEq, Debug)]
struct Token {
    value: String,
    kind: TokenKind,
}

#[derive(Default)]
struct Location {
    line: usize,
    column: usize,
}

#[derive(Default)]
struct Cursor {
    pointer: usize,
    loc: Location,
}

// /semicolonSymbol  symbol = ";"
// asteriskSymbol   symbol = "*"
// commaSymbol      symbol = ","
// leftParenSymbol  symbol = "("
// rightParenSymbol symbol = ")"
// eqSymbol         symbol = "="
// neqSymbol        symbol = "<>"
// neqSymbol2       symbol = "!="
// concatSymbol     symbol = "||"
// plusSymbol       symbol = "+"
// ltSymbol         symbol = "<"
// lteSymbol        symbol = "<="
// gtSymbol         symbol = ">"
// gteSymbol        symbol = ">="
#[derive(Clone, Debug, PartialEq)]
enum SymbolType {
    Semicolon,
    Comma,
    LeftParen,
    RightParen,
    Eq,
    Neq,
    Plus,
    Lt,
    Lte,
    Gt,
    Gte,
    Endl,
    Tab,
}

impl SymbolType {
    pub fn value(&self) -> &str {
        match self {
            SymbolType::Semicolon => ";",
            SymbolType::Comma => ",",
            SymbolType::LeftParen => "(",
            SymbolType::RightParen => ")",
            SymbolType::Eq => "=",
            SymbolType::Neq => "!=",
            SymbolType::Plus => "+",
            SymbolType::Lt => "<",
            SymbolType::Lte => "<=",
            SymbolType::Gt => ">",
            SymbolType::Gte => ">=",
            SymbolType::Endl => "\n",
            SymbolType::Tab => "\t",
        }
    }

    pub fn values() -> Vec<SymbolType> {
        let symbolTypes = [
            SymbolType::Semicolon,
            SymbolType::Comma,
            SymbolType::LeftParen,
            SymbolType::RightParen,
            SymbolType::Eq,
            SymbolType::Neq,
            SymbolType::Plus,
            SymbolType::Lt,
            SymbolType::Lte,
            SymbolType::Gt,
            SymbolType::Gte,
            SymbolType::Endl,
            SymbolType::Tab,
        ];
        symbolTypes.to_vec()
    }
}

#[derive(Clone, Debug, PartialEq)]
enum KeywordType {
    Select,
    From,
    Create,
    Insert,
    As,
    Into,
    Table,
    Values,
    Int,
    Text,
}

impl KeywordType {
    pub fn value(&self) -> &str {
        match self {
            KeywordType::Select => "select",
            KeywordType::From => "from",
            KeywordType::Create => "create",
            KeywordType::Insert => "insert",
            KeywordType::As => "as",
            KeywordType::Table => "table",
            KeywordType::Values => "values",
            KeywordType::Int => "int",
            KeywordType::Text => "text",
            KeywordType::Into => "into",
        }
    }

    pub fn values() -> Vec<KeywordType> {
        let keywordTypes: Vec<KeywordType> = [
            KeywordType::Select,
            KeywordType::From,
            KeywordType::Create,
            KeywordType::Insert,
            KeywordType::As,
            KeywordType::Table,
            KeywordType::Values,
            KeywordType::Into,
            KeywordType::Int,
            KeywordType::Text,
        ]
        .to_vec();
        keywordTypes
    }
}

#[derive(PartialEq, Debug)]
enum TokenKind {
    Keyword(KeywordType),
    Symbol(SymbolType),
    Identifier,
    String,
    Numeric,
    Bool,
    Null,
}

fn lex_symbol(source: &str) -> Option<(Token, Cursor)> {
    let mut cursor = Cursor::default();

    // cases where the cursor should move to the next line.
    if source.starts_with(SymbolType::Endl.value()) || source.starts_with(SymbolType::Tab.value()) {
        cursor.loc.line += 1;
        cursor.loc.column = 0;
    }

    for symbol in SymbolType::values() {
        if source.starts_with(symbol.value()) {
            cursor.pointer = symbol.value().len();
            cursor.loc.column = symbol.value().len();
            return Some((
                Token {
                    value: symbol.value().to_string(),
                    kind: TokenKind::Symbol(symbol),
                },
                cursor,
            ));
        }
    }

    None
}

// TODO: fix case sensitivity.
fn lex_keyword(source: &str) -> Option<(Token, Cursor)> {
    let keyword_match = String::default();
    for keyword in KeywordType::values().iter() {
        if source.starts_with(keyword.value()) {
            let mut cursor = Cursor::default();
            cursor.pointer = keyword.value().len();
            cursor.loc.column = cursor.pointer;
            return Some((
                Token {
                    value: keyword.value().to_string(),
                    kind: TokenKind::Keyword(keyword.clone()),
                },
                cursor,
            ));
        }
    }

    None
}

fn lex_string(source: &str) -> Option<(Token, Cursor)> {
    lex_char_delimited(source, '\'')
}

fn lex_char_delimited(source: &str, delimiter: char) -> Option<(Token, Cursor)> {
    let mut source_iterator = source.char_indices();
    let mut cursor = Cursor::default();

    // the first character should be delimiter.
    if let Some((_, first_char)) = source_iterator.next() {
        if first_char != delimiter {
            return None;
        }
    } else {
        return None;
    }

    let does_sec_delimiter_exist = source_iterator.find(|&(_, c)| c == '\'');
    does_sec_delimiter_exist?;

    let second_delimiter = source_iterator.skip_while(|&(_, c)| c != '\'');

    let mut last_index = source.len();
    for (index, ch) in second_delimiter {
        if ch != delimiter {
            last_index = index - 2;
            break;
        }
    }

    // TODO: this might be wrong.
    cursor.pointer = last_index + 1;
    cursor.loc.column = last_index + 1;

    Some((
        Token {
            value: source[..last_index].to_string(),
            kind: TokenKind::String,
        },
        cursor,
    ))
}

fn lex_numeric(source: &str) -> Option<(Token, Cursor)> {
    let mut cursor = Cursor::default();

    let mut period_found = false;
    let mut exp_marker_found = false;
    let mut exp_marker_index = 0;

    for (i, c) in source.char_indices() {
        cursor.loc.column += 1;

        let is_digit = c.is_digit(10);
        let is_period = c == '.';
        let is_exp_marker = c == 'e';

        // it should start with digit or period.
        if i == 0 && !is_digit && !is_period {
            return None;
        }

        if is_period {
            // there should not be two periods.
            if period_found {
                return None;
            }
            period_found = true;
        } else if is_exp_marker {
            exp_marker_index = i;
            // there should not be two exp markers.
            if exp_marker_found {
                return None;
            }
            exp_marker_found = true;
            // no periods are allowed after the exp marker.
            period_found = true;

            // exp marker should be followed by digits.
            if i == source.len() - 1 {
                return None;
            }
        } else if (c == '+' || c == '-') && exp_marker_index == i - 1 {
            cursor.pointer += 1;
            continue;
        } else if !is_digit {
            break;
        }

        cursor.pointer += 1;
    }

    Some((
        Token {
            value: source[..cursor.pointer].to_string(),
            kind: TokenKind::Numeric,
        },
        cursor,
    ))
}

#[cfg(test)]
mod tests {
    use crate::lexer::TokenKind::Keyword;
    use crate::lexer::{
        lex_char_delimited, lex_keyword, lex_numeric, lex_string, lex_symbol, KeywordType,
        SymbolType, TokenKind,
    };

    fn test_numeric_lexer(source: &str, should_be_none: bool, expected_result: &str) {
        let received_token = lex_numeric(source);
        assert_eq!(received_token.is_none(), should_be_none);
        if !should_be_none {
            if let Some((token, _)) = received_token {
                assert_eq!(token.value, expected_result);
            }
        }
    }

    #[test]
    fn test_lex_numeric_basic_number() {
        let source = "226";
        test_numeric_lexer(source, false, source);
    }

    #[test]
    fn test_lex_numeric_basic_number_one_digit() {
        let source = "8";
        test_numeric_lexer(source, false, source);
    }

    #[test]
    fn test_lex_numeric_exponential_one_digit() {
        let source = "1e3";
        test_numeric_lexer(source, false, source);
    }

    #[test]
    fn test_lex_numeric_exponential_no_exp() {
        let source = "1e";
        test_numeric_lexer(source, true, source);
    }

    #[test]
    fn test_lex_numeric_exponential_negative() {
        let source = "1e-21";
        test_numeric_lexer(source, false, source);
    }

    #[test]
    fn test_lex_numeric_exponential_floating() {
        let source = "1.1e32";
        test_numeric_lexer(source, false, source);
    }

    #[test]
    fn test_lex_numeric_exponential_floating_negative() {
        let source = "1.42e-321";
        test_numeric_lexer(source, false, source);
    }
    #[test]
    fn test_lex_numeric_floating_1() {
        let source = "1.1";
        test_numeric_lexer(source, false, source);
    }

    #[test]
    fn test_lex_numeric_floating_2() {
        let source = ".1";
        test_numeric_lexer(source, false, source);
    }

    #[test]
    fn test_lex_numeric_floating_3() {
        let source = "6.";
        test_numeric_lexer(source, false, source);
    }

    #[test]
    fn test_lex_numeric_exp_no_base() {
        let source = "e8";
        test_numeric_lexer(source, true, source);
    }

    #[test]
    fn test_lex_numeric_exp_two_exp_marks() {
        let source = "1ee7";
        test_numeric_lexer(source, true, source);
    }

    #[test]
    fn test_lex_numeric_floating_two_points() {
        let source = "1..";
        test_numeric_lexer(source, true, source);
    }

    #[test]
    fn test_lex_numeric_invalid_char() {
        let source = "1a1";
        let expected_result = "1";
        test_numeric_lexer(source, false, expected_result);
    }

    #[test]
    fn test_lex_numeric_basic_whitespace() {
        let source = " 1";
        test_numeric_lexer(source, true, source);
    }

    #[test]
    fn test_lex_numeric_end_with_exp_marker() {
        let source = "1e";
        test_numeric_lexer(source, true, source);
    }

    #[test]
    fn test_lex_delimiter_basic() {
        let source = "'aabbcc'";
        let result = lex_char_delimited(source, '\'');
        assert!(result.is_some());
        if let Some((token, _)) = result {
            assert_eq!(token.value, source);
        }
    }

    #[test]
    fn test_lex_delimiter_no_end() {
        let source = "'aabb";
        let result = lex_char_delimited(source, '\'');
        assert!(result.is_none());
    }

    #[test]
    fn test_lex_delimiter_no_start() {
        let source = "asdf'";
        let result = lex_char_delimited(source, '\'');
        assert!(result.is_none());
    }

    #[test]
    fn test_lex_delimiter_escape() {
        let source = "'asdf''";
        let result = lex_char_delimited(source, '\'');
        assert!(result.is_some());
        if let Some((token, _)) = result {
            assert_eq!(token.value, source);
        }
    }

    #[test]
    fn test_lex_delimiter_delimiter_in_between() {
        let source = "'as' 'df''";
        let result = lex_char_delimited(source, '\'');
        let expected = "'as'";
        assert!(result.is_some());
        if let Some((token, _)) = result {
            assert_eq!(token.value, expected);
        }
    }

    #[test]
    fn test_lex_delimiter_delimiter_in_between_2() {
        let source = "'as'x'df''";
        let result = lex_char_delimited(source, '\'');
        let expected = "'as'";
        assert!(result.is_some());
        if let Some((token, _)) = result {
            assert_eq!(token.value, expected);
        }
    }

    #[test]
    fn test_lex_keyword() {
        let source = "into";
        let result = lex_keyword(source);
        assert!(result.is_some());
        if let Some((token, _)) = result {
            assert_eq!(token.value, source);
        }
    }

    #[test]
    fn test_lex_keyword_2() {
        let source = "selectasdf";
        let expected = "select";
        let result = lex_keyword(source);
        assert!(result.is_some());
        if let Some((token, _)) = result {
            assert_eq!(token.value, expected);
            assert_eq!(token.kind, TokenKind::Keyword(KeywordType::Select))
        }
    }

    #[test]
    fn test_lex_keyword_invalid() {
        let source = "Asdf";
        let result = lex_keyword(source);
        assert!(result.is_none());
    }

    #[test]
    fn test_lex_keyword_space_beginning() {
        let source = " select";
        let result = lex_keyword(source);
        assert!(result.is_none());
    }

    #[test]
    fn test_lex_symbol_neq() {
        let source = "!=";
        let result = lex_symbol(source);
        assert!(result.is_some());
        if let Some((token, _)) = result {
            assert_eq!(token.value, source);
            assert_eq!(token.kind, TokenKind::Symbol(SymbolType::Neq));
        }
    }

    #[test]
    fn test_lex_symbol_space_beginning() {
        let source = " !=";
        let result = lex_symbol(source);
        assert!(result.is_none());
    }
}
