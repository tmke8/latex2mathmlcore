use std::fmt;

use memchr::memmem::Finder;

use latex2mmlc::{Display, LatexError};

#[derive(Debug)]
pub enum ConversionError<'source> {
    UnclosedDelimiter,
    NestedDelimiters,
    MismatchedDelimiters,
    LatexError(LatexError<'source>, &'source str),
}
impl fmt::Display for ConversionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::UnclosedDelimiter => write!(f, "Unclosed delimiter"),
            ConversionError::NestedDelimiters => write!(f, "Nested delimiters are not allowed"),
            ConversionError::MismatchedDelimiters => write!(f, "Unmatched delimiters"),
            ConversionError::LatexError(e, input) => {
                write!(f, "Error at {} in '{}':\n{}", e.0, input, e)
            }
        }
    }
}
impl std::error::Error for ConversionError<'_> {}

pub struct Replacer<'config> {
    opening_finders: (Finder<'config>, Finder<'config>),
    closing_finders: (Finder<'config>, Finder<'config>),
    opening_lengths: (usize, usize),
    closing_lengths: (usize, usize),
}

impl<'config> Replacer<'config> {
    pub fn new(
        inline_delim: (&'config str, &'config str),
        block_delim: (&'config str, &'config str),
    ) -> Self {
        let inline_opening = Finder::new(inline_delim.0);
        let inline_closing = Finder::new(inline_delim.1);
        let block_opening = Finder::new(block_delim.0);
        let block_closing = Finder::new(block_delim.1);

        Self {
            opening_finders: (inline_opening, block_opening),
            closing_finders: (inline_closing, block_closing),
            opening_lengths: (inline_delim.0.len(), block_delim.0.len()),
            closing_lengths: (inline_delim.1.len(), block_delim.1.len()),
        }
    }

    /// Replaces the content of inline and block math delimiters in a LaTeX string.
    ///
    /// Any kind of nesting of delimiters is not allowed.
    #[inline]
    pub(crate) fn replace<'source, F>(
        &self,
        input: &'source str,
        f: F,
    ) -> Result<String, ConversionError<'source>>
    where
        F: for<'buf> Fn(&'buf mut String, &'source str, Display) -> Result<(), LatexError<'source>>,
    {
        let mut result = String::with_capacity(input.len());
        let mut current_pos = 0;

        while current_pos < input.len() {
            let remaining = &input[current_pos..];

            // Find the next occurrence of any opening delimiter
            let opening = self.find_next_delimiter(remaining, true);

            let Some((open_typ, idx)) = opening else {
                // No more opening delimiters found
                result.push_str(remaining);
                break;
            };

            let opening_delim_len = match open_typ {
                Display::Inline => self.opening_lengths.0,
                Display::Block => self.opening_lengths.1,
            };

            let start = current_pos + idx;
            // Append everything before the opening delimiter
            result.push_str(&input[current_pos..start]);
            // Skip the opening delimiter itself
            let start = start + opening_delim_len;
            let remaining = &input[start..];

            // Find the next occurrence of any closing delimiter
            let closing = self.find_next_delimiter(remaining, false);

            let Some((close_typ, idx)) = closing else {
                // No closing delimiter found
                return Err(ConversionError::UnclosedDelimiter);
            };

            let closing_delim_len = match close_typ {
                Display::Inline => self.closing_lengths.0,
                Display::Block => self.closing_lengths.1,
            };

            if open_typ != close_typ {
                // Mismatch of opening and closing delimiter
                return Err(ConversionError::MismatchedDelimiters);
            }

            let end = start + idx;
            // Get the content between delimiters
            let content = &input[start..end];
            // Check whether any *opening* delimiters are present in the content
            if self.find_next_delimiter(content, true).is_some() {
                return Err(ConversionError::NestedDelimiters);
            }
            // Convert the content
            f(&mut result, content, open_typ)
                .map_err(|e| ConversionError::LatexError(e, content))?;
            // Update current position
            current_pos = end + closing_delim_len;
        }

        Ok(result)
    }

    /// Finds the next occurrence of either an inline or block delimiter.
    fn find_next_delimiter(&self, input: &str, opening: bool) -> Option<(Display, usize)> {
        let (inline_finder, block_finder) = if opening {
            (&self.opening_finders.0, &self.opening_finders.1)
        } else {
            (&self.closing_finders.0, &self.closing_finders.1)
        };

        let inline_pos = inline_finder.find(input.as_bytes());
        let block_pos = block_finder.find(input.as_bytes());

        match (inline_pos, block_pos) {
            // If we have i == d, Display has priority
            (Some(i), Some(d)) if i < d => Some((Display::Inline, i)),
            (_, Some(d)) => Some((Display::Block, d)),
            (Some(i), None) => Some((Display::Inline, i)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    /// Mock convert function for testing
    fn convert(
        buf: &mut String,
        content: &'static str,
        typ: Display,
    ) -> Result<(), LatexError<'static>> {
        match typ {
            Display::Inline => write!(buf, "[T1:{}]", content).unwrap(),
            Display::Block => write!(buf, "[T2:{}]", content).unwrap(),
        };
        Ok(())
    }

    fn replace(
        input: &'static str,
        inline_delim: (&str, &str),
        block_delim: (&str, &str),
    ) -> Result<String, ConversionError<'static>> {
        let replacer = Replacer::new(inline_delim, block_delim);
        replacer.replace(input, |buf, content, typ| convert(buf, content, typ))
    }

    #[test]
    fn test_basic_replacement() {
        let input = "Hello $world$ and $$universe$$";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap();
        assert_eq!(result, "Hello [T1:world] and [T2:universe]");
    }

    #[test]
    fn test_nested_delimiters() {
        let input = "Nested $$outer $inner$ delimiter$$";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap_err();
        assert!(matches!(result, ConversionError::MismatchedDelimiters));
    }

    #[test]
    fn test_nested_delimiters2() {
        let input = "Nested $outer $$inner$$ delimiter$";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap_err();
        assert!(matches!(result, ConversionError::MismatchedDelimiters));
    }

    #[test]
    fn test_mismatched_unclosed() {
        let input = "Unclosed $delimiter";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap_err();
        assert!(matches!(result, ConversionError::UnclosedDelimiter));
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_no_delimiters() {
        let input = "Hello, world!";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_multiple_replacements() {
        let input = "$a$ then $$b$$ then $c$ and $$d$$";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap();
        assert_eq!(result, "[T1:a] then [T2:b] then [T1:c] and [T2:d]");
    }

    #[test]
    fn test_complete_replacements() {
        let input = "$a then b then c and d$";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap();
        assert_eq!(result, "[T1:a then b then c and d]");
    }

    #[test]
    fn test_mismatched_delimiters() {
        let input = "Mismatch $$ and $ signs";
        let result = replace(input, ("$", "$"), ("$$", "$$"));
        assert!(matches!(result, Err(ConversionError::MismatchedDelimiters)));
    }

    #[test]
    fn test_identical_delimiters() {
        let input = "|a| and ||b||";
        let result = replace(input, ("|", "|"), ("||", "||")).unwrap();
        assert_eq!(result, "[T1:a] and [T2:b]");
    }

    #[test]
    fn test_asymmetric_delimiters() {
        let input = r"let \(a=1\) and \[b=2\].";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]")).unwrap();
        assert_eq!(result, "let [T1:a=1] and [T2:b=2].");
    }

    #[test]
    fn test_asymmetric_delimiters_partial_delim() {
        let input = r"let\ \(a=1\) and \[b=2\].";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]")).unwrap();
        assert_eq!(result, "let\\ [T1:a=1] and [T2:b=2].");
    }

    #[test]
    fn test_asymmetric_delimiters_nested() {
        let input = r"let \(a=1 and \[b=2\]\).";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]"));
        assert!(matches!(result, Err(ConversionError::MismatchedDelimiters)));
    }

    #[test]
    fn test_asymmetric_delimiters_nested2() {
        let input = r"let \(a=1 and \[b=2\).";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]"));
        assert!(matches!(result, Err(ConversionError::NestedDelimiters)));
    }

    #[test]
    fn test_asymmetric_delimiters_nested3() {
        let input = r"let \(a=1 and \(b=2\).";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]"));
        assert!(matches!(result, Err(ConversionError::NestedDelimiters)));
    }

    #[test]
    fn test_multibyte_delimiters() {
        let input = "this is über ü(a=2ü).";
        let result = replace(input, ("ü(", "ü)"), ("ü[", "ü]")).unwrap();
        assert_eq!(result, "this is über [T1:a=2].");
    }
}
