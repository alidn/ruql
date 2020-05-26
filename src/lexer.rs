use std::fs::OpenOptions;

#[derive(PartialEq, Debug)]
struct Token {
    value: String,
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

fn lex_delimiter(source: &str, delimiter: char) -> Option<(Token, Cursor)> {
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

    let find_sec_delimiter_result = source_iterator.find(|&(_, c)| c == '\'');
    find_sec_delimiter_result?;

    let second_delimiter = source_iterator.skip_while(|&(_, c)| c != '\'');
    let mut last_delimiter = second_delimiter.skip_while(|&(_, c)| c == '\'');

    let last_index = match last_delimiter.next() {
        Some((index, _)) => index - 1,
        None => source.len() - 1,
    } + 1;

    cursor.pointer = last_index + 1;
    cursor.loc.column = last_index + 1;

    Some((
        Token {
            value: source[..last_index].to_string(),
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
        },
        cursor,
    ))
}

#[cfg(test)]
mod tests {
    use crate::lexer::{lex_delimiter, lex_numeric};

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
    fn trest_lex_numeric_basic_number_one_digit() {
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
        let result = lex_delimiter(source, '\'');
        assert!(result.is_some());
        if let Some((token, _)) = result {
            assert_eq!(token.value, source);
        }
    }

    #[test]
    fn test_lex_delimiter_no_end() {
        let source = "'aabbcc";
        let result = lex_delimiter(source, '\'');
        assert!(result.is_none());
    }
}
