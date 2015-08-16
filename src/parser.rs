use std::num;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(char),
    UnexpectedEnd,
    InvalidNumber(String, num::ParseIntError),
}

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<'src> {
    source: &'src str,
    position: usize,
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
    Number(u64),
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Parser<'src> {
        Parser {
            source: source,
            position: 0,
        }
    }

    fn parse_expr(&mut self) -> ParseResult<Expr> {
        let c = try!(self.peek_char().ok_or(ParseError::UnexpectedEnd));

        match c {
            '(' => unimplemented!(),
            c if is_ident_start(c) => Ok(self.parse_ident()),
            c if is_digit(c)       => self.parse_number(),
            _                      => Err(ParseError::UnexpectedChar(c)),
        }
    }

    fn parse_number(&mut self) -> ParseResult<Expr> {
        let mut number = String::new();

        while let Some(digit) = self.read_char() {
            // Eagerly read all valid identifier characters so that "123foo" doesn't lex as a
            // number followed by an identifier.
            if !is_ident_char(digit) {
                self.unread_char();
                break;
            }

            number.push(digit);
        }

        match number.parse::<u64>() {
            Ok(value) => Ok(Expr::Number(value)),
            Err(reason) => Err(ParseError::InvalidNumber(number, reason)),
        }
    }

    fn parse_ident(&mut self) -> Expr {
        let mut ident = String::new();

        while let Some(c) = self.read_char() {
            if !is_ident_char(c) {
                self.unread_char();
                break;
            }

            ident.push(c);
        }

        Expr::Ident(ident)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.read_char() {
            if !is_whitespace(c) {
                self.unread_char();
                break;
            }
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    fn read_char(&mut self) -> Option<char> {
        let opt_c = self.peek_char();

        if let Some(c) = opt_c {
            self.position += c.len_utf8();
        }

        opt_c
    }

    /// Step backwards one `char` in the input. Must not be called more times than `read_char` has
    /// been called.
    fn unread_char(&mut self) {
        assert!(self.position != 0);
        let (prev_pos, _) = self.source[..self.position].char_indices().next_back().unwrap();
        self.position = prev_pos;
    }

    fn at_end(&self) -> bool {
        self.position == self.source.len()
    }
}

impl<'src> Iterator for Parser<'src> {
    type Item = ParseResult<Expr>;

    fn next(&mut self) -> Option<ParseResult<Expr>> {
        self.skip_whitespace();

        if self.at_end() {
            None
        } else {
            Some(self.parse_expr())
        }
    }
}

/// Returns `true` if the given character is whitespace.
fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\t' | '\n' => true,
        _                 => false,
    }
}

/// Returns `true` if the given character is a digit.
fn is_digit(c: char) -> bool {
    match c {
        '0'...'9' => true,
        _         => false,
    }
}

/// Returns `true` if the given character is valid at the start of an identifier.
fn is_ident_start(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '_' | '!' | '?' | '*' | '-' | '+' | '/' | '=' | '<' | '>'
            => true,
            _ => false,
    }
}

/// Returns `true` if the given character is valid in an identifier.
fn is_ident_char(c: char) -> bool {
    is_ident_start(c) || is_digit(c)
}
