#[derive(PartialEq)]
pub enum Lexeme {
    Num(f32),
    Plus,
    Minus,
    Times,
    Divide,
    LeftParen,
    RightParen,
}

impl Lexeme {
    pub fn lex_line(input: &str) -> Result<Vec<Lexeme>, String> {
        let mut lexemes = Vec::new();
        let mut stack = String::new();

        let push_lexeme = |ls: &mut Vec<Lexeme>, s: &mut String| -> Result<(), String> {
            if !s.is_empty() {
                if let Ok(num) = s.parse::<f32>() {
                    ls.push(Lexeme::Num(num));
                    s.clear();
                } else {
                    return Err("Unrecognized Token".into());
                }
            }

            Ok(())
        };

        for c in input.chars() {
            match c {
                '+' => {
                    push_lexeme(&mut lexemes, &mut stack)?;
                    lexemes.push(Lexeme::Plus);
                }
                '-' => {
                    push_lexeme(&mut lexemes, &mut stack)?;
                    lexemes.push(Lexeme::Minus);
                }
                '*' => {
                    push_lexeme(&mut lexemes, &mut stack)?;
                    lexemes.push(Lexeme::Times);
                }
                '/' => {
                    push_lexeme(&mut lexemes, &mut stack)?;
                    lexemes.push(Lexeme::Divide);
                }
                '(' => {
                    push_lexeme(&mut lexemes, &mut stack)?;
                    lexemes.push(Lexeme::LeftParen);
                }
                ')' => {
                    push_lexeme(&mut lexemes, &mut stack)?;
                    lexemes.push(Lexeme::RightParen);
                }
                _ => stack.push(c)
            }
        }

        push_lexeme(&mut lexemes, &mut stack)?;

        Ok(lexemes)
    }
}
