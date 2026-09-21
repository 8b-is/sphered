#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    LAngle,
    RAngle,
    Question,
    Bang,
    Pipe,
    At,
    Hash,
    Tilde,
    Atom(String),
}

fn is_op(c: char) -> bool {
    matches!(c, '(' | ')' | '<' | '>' | '?' | '!' | '|' | '@' | '#' | '~')
}

pub fn lex(src: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];

        let single = match c {
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '<' => Some(Token::LAngle),
            '>' => Some(Token::RAngle),
            '?' => Some(Token::Question),
            '!' => Some(Token::Bang),
            '|' => Some(Token::Pipe),
            '@' => Some(Token::At),
            '#' => Some(Token::Hash),
            '~' => Some(Token::Tilde),
            _ => None,
        };
        if let Some(t) = single {
            out.push(t);
            i += 1;
            continue;
        }

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        let start = i;
        while i < chars.len() && !chars[i].is_whitespace() && !is_op(chars[i]) {
            i += 1;
        }
        if i == start {
            return Err(format!("unexpected character {c:?}"));
        }
        out.push(Token::Atom(chars[start..i].iter().collect()));
    }
    Ok(out)
}
