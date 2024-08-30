//! Lexer
//!
//! - Input: `String`
//! - Output: `Vec<Token>`
//!

use std::mem;
use std::num::NonZero;
use std::str::CharIndices;

use crate::attribute::ParenAttr;
use crate::commands::get_command;
use crate::error::GetUnwrap;
use crate::{ops, token::Token};

/// Lexer
#[derive(Debug, Clone)]
pub(crate) struct Lexer<'source> {
    input: CharIndices<'source>,
    peek: (usize, char),
    input_string: &'source str,
    pub input_length: usize,
    pub text_mode: bool,
}

impl<'source> Lexer<'source> {
    /// Receive the input source code and generate a LEXER instance.
    pub(crate) fn new(input: &'source str) -> Self {
        let mut lexer = Lexer {
            input: input.char_indices(),
            peek: (0, '\u{0}'),
            input_string: input,
            input_length: input.len(),
            text_mode: false,
        };
        lexer.read_char(); // Initialize `peek`.
        lexer
    }

    /// One character progresses.
    fn read_char(&mut self) -> (usize, char) {
        mem::replace(
            &mut self.peek,
            self.input.next().unwrap_or((self.input_length, '\u{0}')),
        )
    }

    /// Skip whitespace characters.
    fn skip_whitespace(&mut self) -> Option<NonZero<usize>> {
        let mut skipped = None;
        while self.peek.1.is_ascii_whitespace() {
            let (loc, _) = self.read_char();
            // This is technically wrong because there can be whitespace at position 0,
            // but we are only recording whitespace in text mode, which is started by
            // the `\text` command, so at position 0 we will never we in text mode.
            skipped = NonZero::<usize>::new(loc);
        }
        skipped
    }

    /// Read one command.
    #[inline]
    fn read_command(&mut self) -> &'source str {
        let start = self.peek.0;

        // Read in all ASCII characters.
        while self.peek.1.is_ascii_alphabetic() {
            self.read_char();
        }

        if start == self.peek.0 {
            // Always read at least one character.
            self.read_char();
        }

        // To get the end of the command, we take the index of the next character.
        let end = self.peek.0;
        // SAFETY: we got `start` and `end` from `CharIndices`, so they are valid bounds.
        self.input_string.get_unwrap(start..end)
    }

    /// Read one number.
    #[inline]
    fn read_number(&mut self, start: usize) -> Token<'source> {
        while {
            let cur = self.peek.1;
            cur.is_ascii_digit() || Punctuation::from_char(cur).is_some()
        } {
            let (index_before, candidate) = self.read_char();
            // Before we accept the current character, we need to check the next one.
            if !self.peek.1.is_ascii_digit() {
                if let Some(punctuation) = Punctuation::from_char(candidate) {
                    // If the candidate is punctuation and the next character is not a digit,
                    // we don't want to include the punctuation.
                    // But we do need to return the punctuation as an operator.
                    let number = self.input_string.get_unwrap(start..index_before);
                    return match punctuation {
                        Punctuation::Dot => Token::NumberWithDot(number, start as _),
                        Punctuation::Comma => Token::NumberWithComma(number, start as _),
                    };
                }
            }
        }
        let end = self.peek.0;
        let number = self.input_string.get_unwrap(start..end);
        Token::Number(number, start as _)
    }

    /// Read text until the next `}`.
    #[inline]
    pub(crate) fn read_text_content(&mut self) -> Option<&'source str> {
        let mut brace_count = 1;
        let start = self.peek.0;

        loop {
            let (end, cur) = self.read_char();
            if cur == '{' {
                brace_count += 1;
            } else if cur == '}' {
                brace_count -= 1;
            }
            if brace_count <= 0 {
                return Some(self.input_string.get_unwrap(start..end));
            }
            // Check for escaped characters.
            if cur == '\\' {
                let (_, cur) = self.read_char();
                // We only allow \{ and \} as escaped characters.
                if !matches!(cur, '{' | '}') {
                    return None;
                }
            }
            if cur == '\u{0}' {
                return None;
            }
        }
    }

    /// Generate the next token.
    pub(crate) fn next_token(&mut self, wants_digit: bool) -> Token<'source> {
        if let Some(loc) = self.skip_whitespace() {
            if self.text_mode {
                return Token::Whitespace(loc.get() as _);
            }
        }
        if wants_digit && self.peek.1.is_ascii_digit() {
            let (start, _) = self.read_char();
            let end = self.peek.0;
            let num = self.input_string.get_unwrap(start..end);
            return Token::Number(num, start as _);
        }

        let (loc, ch) = self.read_char();
        let tok = match ch {
            '=' => Token::Operator(ops::EQUALS_SIGN, loc as _),
            ';' => Token::Operator(ops::SEMICOLON, loc as _),
            ',' => Token::Operator(ops::COMMA, loc as _),
            '\'' => Token::Prime(loc as _),
            '(' => Token::Paren(ops::LEFT_PARENTHESIS, None, loc as _),
            ')' => Token::Paren(ops::RIGHT_PARENTHESIS, None, loc as _),
            '{' => Token::GroupBegin(loc as _),
            '}' => Token::GroupEnd(loc as _),
            '[' => Token::Paren(ops::LEFT_SQUARE_BRACKET, None, loc as _),
            ']' => Token::SquareBracketClose(loc as _),
            '|' => Token::Paren(ops::VERTICAL_LINE, Some(ParenAttr::Ordinary), loc as _),
            '+' => Token::Operator(ops::PLUS_SIGN, loc as _),
            '-' => Token::Operator(ops::MINUS_SIGN, loc as _),
            '*' => Token::Operator(ops::ASTERISK, loc as _),
            '!' => Token::Operator(ops::EXCLAMATION_MARK, loc as _),
            '<' => Token::OpLessThan(loc as _),
            '>' => Token::OpGreaterThan(loc as _),
            '_' => Token::Underscore(loc as _),
            '^' => Token::Circumflex(loc as _),
            '&' => Token::Ampersand(loc as _),
            '~' => Token::NonBreakingSpace(loc as _),
            '\u{0}' => Token::EOF(loc as _),
            ':' => Token::Colon(loc as _),
            ' ' => Token::Letter('\u{A0}', loc as _),
            '\\' => {
                let cmd = get_command(self.read_command());
                if self.text_mode {
                    // After a command, all whitespace is skipped, even in text mode.
                    self.skip_whitespace();
                }
                cmd
            }
            c => {
                if c.is_ascii_digit() {
                    self.read_number(loc)
                } else if c.is_ascii_graphic() {
                    // Some symbols like '.' and '/' are considered operators by the MathML Core spec,
                    // but in LaTeX they behave like normal identifiers (they are in the "ordinary" class 0).
                    // One might think that they could be rendered as `<mo>` with custom spacing,
                    // but then they still interact with other operators in ways that are not correct.
                    Token::Letter(c, loc as _)
                } else {
                    Token::NormalLetter(c, loc as _)
                }
            }
        };
        tok
    }
}

enum Punctuation {
    Dot,
    Comma,
}

impl Punctuation {
    fn from_char(c: char) -> Option<Self> {
        match c {
            ops::FULL_STOP => Some(Self::Dot),
            ',' => Some(Self::Comma),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::token::Token;
    use super::*;

    #[test]
    fn lexer_test() {
        let problems = vec![
            (r"3", vec![Token::Number("3", 0)]),
            (r"3.14", vec![Token::Number("3.14", 0)]),
            (r"3.14.", vec![Token::NumberWithDot("3.14", 0)]),
            (
                r"3..14",
                vec![
                    Token::NumberWithDot("3", 0),
                    Token::Letter('.', 2),
                    Token::Number("14", 3),
                ],
            ),
            (r"x", vec![Token::Letter('x', 0)]),
            (r"\pi", vec![Token::Letter('π', 0)]),
            (
                r"x = 3.14",
                vec![
                    Token::Letter('x', 0),
                    Token::Operator(ops::EQUALS_SIGN, 2),
                    Token::Number("3.14", 4),
                ],
            ),
            (r"\alpha\beta", vec![Token::Letter('α', 0), Token::Letter('β', 6)]),
            (
                r"x+y",
                vec![
                    Token::Letter('x', 0),
                    Token::Operator(ops::PLUS_SIGN, 1),
                    Token::Letter('y', 2),
                ],
            ),
            (r"\ 1", vec![Token::Space("1", 0), Token::Number("1", 2)]),
        ];

        for (problem, answer) in problems.iter() {
            let mut lexer = Lexer::new(problem);
            for answer in answer.iter() {
                assert_eq!(&lexer.next_token(false), answer);
            }
        }
    }
}
