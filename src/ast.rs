type Name = String;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Invocation(Name, Vec<Expression>),
    StringLiteral(String),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Func,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_func_keyword() {
        let keyword_parser = parser::KeywordParser::new();
        let mut errors = vec![];
        let keyword = keyword_parser.parse(&mut errors, "func").unwrap();
        assert_eq!(keyword, Keyword::Func);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn parses_double_quoted_string_literal() {
        let string_parser = parser::StringLiteralParser::new();
        let mut errors = vec![];
        let string = string_parser
            .parse(&mut errors, r#""hello world""#)
            .unwrap();
        assert_eq!(string, "hello world");
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn parses_escaped_string_literal() {
        let string_parser = parser::StringLiteralParser::new();
        let mut errors = vec![];
        let string = string_parser
            .parse(&mut errors, r#""hello\"\nworld""#)
            .unwrap();
        assert_eq!(string, "hello\"\nworld");
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn parses_println_invocation() {
        let expr_parser = parser::ExpressionParser::new();
        let mut errors = vec![];
        let invocation = expr_parser
            .parse(&mut errors, r#"println("asdf")"#)
            .unwrap();
        assert_eq!(
            invocation,
            Expression::Invocation(
                "println".into(),
                vec![Expression::StringLiteral("asdf".into())]
            )
        );
        assert_eq!(errors.len(), 0);
    }
}
