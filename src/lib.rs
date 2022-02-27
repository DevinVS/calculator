use lexer::Lexeme;
use parser::{Parse, Expr};

mod lexer;
pub mod parser;

pub fn parse(input: &str) -> Option<Expr> {
    if let Ok(lexemes) = Lexeme::lex_line(input) {
        if let Some((tree, _)) = Expr::parse(lexemes.as_slice()) {
            return Some(tree)
        }
    }

    None
}
