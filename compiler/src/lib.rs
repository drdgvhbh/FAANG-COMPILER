use lalrpop_util::ParseError;

use codespan::{FileId, Span};
use codespan_reporting::diagnostic;
use codespan_reporting::diagnostic::Diagnostic;

pub mod ast;
pub mod compiler;
#[allow(dead_code)]
pub mod parser;

pub fn parse(text: &str, file_id: FileId) -> Result<ast::Expression, Vec<Diagnostic>> {
    let program_parser = parser::ExpressionParser::new();
    let mut errors = vec![];
    let program_result = program_parser.parse(&mut errors, &text);
    let mut diagnostics = vec![];

    if errors.len() > 0 || program_result.is_err() {
        if program_result.is_err() {
            errors.push(program_result.unwrap_err());
        }

        for err in &errors {
            match err {
                ParseError::UnrecognizedToken { token, expected } => {
                    let (start, input, end) = token;
                    let label = diagnostic::Label::new(
                        file_id,
                        Span::new((*start) as u32, (*end) as u32),
                        format!("expected one of {} here", expected.join(", ")),
                    );
                    let diagnostic = diagnostic::Diagnostic::new_error(
                        format!("unrecognized token: {}", input),
                        label,
                    );
                    diagnostics.push(diagnostic);
                }
                ParseError::UnrecognizedEOF { location, expected } => {
                    let start = location;
                    let end = start;
                    let label = diagnostic::Label::new(
                        file_id,
                        Span::new((*start) as u32, (*end) as u32),
                        format!("expected one of {} here", expected.join(", ")),
                    );
                    let diagnostic =
                        diagnostic::Diagnostic::new_error(format!("unrecognized EOF"), label);
                    diagnostics.push(diagnostic);
                }
                ParseError::InvalidToken { location } => {
                    let start = location;
                    let end = start;
                    let label = diagnostic::Label::new(
                        file_id,
                        Span::new((*start) as u32, (*end) as u32),
                        "invalid token",
                    );
                    let diagnostic =
                        diagnostic::Diagnostic::new_error(format!("invalid token"), label);
                    diagnostics.push(diagnostic);
                }
                ParseError::ExtraToken { token } => {
                    let (start, input, end) = token;
                    let label = diagnostic::Label::new(
                        file_id,
                        Span::new((*start) as u32, (*end) as u32),
                        "unrecognized token",
                    );
                    let diagnostic = diagnostic::Diagnostic::new_error(
                        format!("unrecognized token: {}", input),
                        label,
                    );
                    diagnostics.push(diagnostic);
                }
                _ => {}
            }
        }

        return Err(diagnostics);
    }

    Ok(program_result.unwrap())
}
