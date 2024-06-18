use std::mem::discriminant;

use strum_macros::AsRefStr;

use crate::attribute::{FracAttr, Style, TextTransform};
use crate::ops::Op;

#[derive(Debug, Clone, PartialEq, AsRefStr)]
#[repr(u32)]
pub enum Token<'source> {
    #[strum(serialize = "end of document")]
    EOF,
    #[strum(serialize = r"\begin{...}")]
    Begin,
    #[strum(serialize = r"\end{...}")]
    End,
    #[strum(serialize = "&")]
    Ampersand,
    #[strum(serialize = r"\\")]
    NewLine,
    #[strum(serialize = r"\left")]
    Left,
    #[strum(serialize = r"\right")]
    Right,
    Middle,
    Paren(Op),
    /// The closing square bracket has its own token because we often
    /// need to search for it.
    #[strum(serialize = "]")]
    SquareBracketClose,
    #[strum(serialize = "{")]
    GroupBegin,
    #[strum(serialize = "}")]
    GroupEnd,
    Frac(Option<FracAttr>),
    #[strum(serialize = r"\genfrac")]
    Genfrac,
    #[strum(serialize = "_")]
    Underscore,
    #[strum(serialize = "^")]
    Circumflex,
    Binom(Option<FracAttr>),
    Overset,
    Underset,
    Overbrace(Op),
    Underbrace(Op),
    #[strum(serialize = r"\sqrt")]
    Sqrt,
    Integral(Op),
    #[strum(serialize = r"\limits")]
    Limits,
    Lim(&'static str),
    Space(&'static str),
    #[strum(serialize = "~")]
    NonBreakingSpace,
    Transform(TextTransform),
    NormalVariant,
    Big(&'static str),
    Over(Op),
    Under(Op),
    Operator(Op),
    #[strum(serialize = "'")]
    Prime,
    #[strum(serialize = ">")]
    OpGreaterThan,
    #[strum(serialize = "<")]
    OpLessThan,
    #[strum(serialize = r"\&")]
    OpAmpersand,
    #[strum(serialize = ":")]
    Colon,
    BigOp(Op),
    Letter(char),
    NormalLetter(char),
    Number(&'source str),
    NumberWithDot(&'source str),
    NumberWithComma(&'source str),
    Function(&'static str),
    #[strum(serialize = r"\operatorname")]
    OperatorName,
    Slashed,
    #[strum(serialize = r"\not")]
    Not,
    #[strum(serialize = r"\text")]
    Text,
    #[strum(serialize = r"\mathstrut")]
    Mathstrut,
    Style(Style),
    UnknownCommand(&'source str),
}

impl Token<'_> {
    pub(crate) fn acts_on_a_digit(&self) -> bool {
        matches!(
            self,
            Token::Sqrt | Token::Frac(_) | Token::Binom(_) | Token::Transform(_)
        )
    }

    /// Returns `true` if `self` and `other` are of the same kind.
    /// Note that this does not compare the content of the tokens.
    pub(crate) fn is_same_kind(&self, other: &Token) -> bool {
        discriminant(self) == discriminant(other)
    }
}
