//! latex2mmlc
//!
//! Provides a functionality to convert LaTeX math equations to MathML representation.
//! This crate is implemented in pure Rust, so it works for all platforms including WebAssembly.
//!
//! # Supported LaTeX commands
//!
//! - Numbers, e.g. `0`, `3.14`, ...
//! - ASCII and Greek (and more) letters, e.g. `x`, `\alpha`, `\pi`, `\aleph`, ...
//! - Symbols, e.g., `\infty`, `\dagger`, `\angle`, `\Box`, `\partial`, ...
//! - Binary relations, e.g. `=`, `>`, `<`, `\ll`, `:=`, ...
//! - Binary operations, e.g. `+`. `-`, `*`, `/`, `\times`, `\otimes`, ...
//! - Basic LaTeX commands, e.g. `\sqrt`, `\frac`, `\sin`, `\binom`, ...
//! - Parentheses, e.g., `\left\{ .. \middle| .. \right]`, ...
//! - Integrals, e.g., `\int_0^\infty`, `\iint`, `\oint`, ...
//! - Big operators, e.g., `\sum`, `\prod`, `\bigcup_{i = 0}^\infty`, ...
//! - Limits and overset/underset, e.g., `\lim`, `\overset{}{}`, `\overbrace{}{}`, ...
//! - Font styles, e.g. `\mathrm`, `\mathbf`, `\bm`, `\mathit`, `\mathsf`, `\mathscr`, `\mathbb`, `\mathfrak`, `\texttt`.
//!   - MathML lacks calligraphic mathvariant: https://github.com/mathml-refresh/mathml/issues/61
//! - White spaces, e.g., `\!`, `\,`, `\:`, `\;`, `\ `, `\quad`, `\qquad`.
//! - Matrix, e.g. `\begin{matrix}`, `\begin{pmatrix}`, `\begin{bmatrix}`, `\begin{vmatrix}`.
//! - Multi-line equation `\begin{align}` (experimental).
//! - Feynman slash notation: `\slashed{\partial}`.
//!
//! ## Unsupported LaTeX commands
//!
//! - New line `\\`, except for ones in a matrix or align environment.
//! - Alignment `&`, except for ones in a matrix or align environment.
//! - Complicated sub/superscripts (`<mmultiscripts>`).
//!
//!
//! # Usage
//!
//!  Main functions of this crate are  [`latex_to_mathml`](./fn.latex_to_mathml.html) and
//! [`replace`](./fn.replace.html).
//!
//! ```rust
//! use latex2mmlc::{latex_to_mathml, Display};
//!
//! let latex = r#"\erf ( x ) = \frac{ 2 }{ \sqrt{ \pi } } \int_0^x e^{- t^2} \, dt"#;
//! let mathml = latex_to_mathml(latex, Display::Block, true).unwrap();
//! println!("{}", mathml);
//! ```
//!
//! For more examples and list of supported LaTeX commands, please check
//! [`examples/equations.rs`](https://github.com/osanshouo/latex2mathml/blob/master/examples/equations.rs)
//! and [`examples/document.rs`](https://github.com/osanshouo/latex2mathml/blob/master/examples/document.rs).
//!

use bumpalo::{collections::String, Bump};

pub mod ast;
pub mod attribute;
mod error;
pub(crate) mod lexer;
pub(crate) mod ops;
pub(crate) mod parse;
pub mod token;
pub use error::LatexError;

/// display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Block,
    Inline,
}

fn get_nodes<'a>(alloc: &'a Bump, latex: &'a str) -> Result<ast::Node<'a>, error::LatexError<'a>> {
    let l = lexer::Lexer::new(latex, alloc);
    let mut p = parse::Parser::new(l, alloc);
    p.parse()
}

/// Convert LaTeX text to MathML.
///
/// The second argument specifies whether it is inline-equation or block-equation.
///
/// ```rust
/// use latex2mmlc::{latex_to_mathml, Display};
///
/// let latex = r#"(n + 1)! = \Gamma ( n + 1 )"#;
/// let mathml = latex_to_mathml(latex, Display::Inline, true).unwrap();
/// println!("{}", mathml);
///
/// let latex = r#"x = \frac{ - b \pm \sqrt{ b^2 - 4 a c } }{ 2 a }"#;
/// let mathml = latex_to_mathml(latex, Display::Block, true).unwrap();
/// println!("{}", mathml);
/// ```
///
pub fn latex_to_mathml<'a>(
    alloc: &'a Bump,
    latex: &'a str,
    display: Display,
    pretty: bool,
) -> Result<String<'a>, error::LatexError<'a>> {
    let nodes = get_nodes(alloc, latex)?;

    let mut output = match display {
        Display::Block => String::from_str_in("<math display=\"block\">", alloc),
        Display::Inline => String::from_str_in("<math>", alloc),
    };

    nodes.emit(&mut output, if pretty { 1 } else { 0 });
    if pretty {
        output.push('\n');
    }
    output.push_str("</math>");
    Ok(output)
}

#[cfg(test)]
mod tests {
    use bumpalo::Bump;
    use insta::assert_snapshot;

    use crate::latex_to_mathml;

    use super::get_nodes;

    #[test]
    fn full_tests() {
        let problems = vec![
            ("integer", r"0"),
            ("rational_number", r"3.14"),
            ("single_variable", r"x"),
            ("greek_letter", r"\alpha"),
            ("greek_letters", r"\phi/\varphi"),
            ("simple_expression", r"x = 3+\alpha"),
            ("sine_function", r"\sin x"),
            ("square_root", r"\sqrt 2"),
            ("square_root_without_space", r"\sqrt12"),
            ("complex_square_root", r"\sqrt{x+2}"),
            ("cube_root", r"\sqrt[3]{x}"),
            ("simple_fraction", r"\frac{1}{2}"),
            ("fraction_without_space", r"\frac12"),
            ("slightly_more_complex_fraction", r"\frac{12}{5}"),
            ("superscript", r"x^2"),
            ("double_subscript", r"g_{\mu\nu}"),
            ("simple_accent", r"\dot{x}"),
            ("operator_name", r"\operatorname{sn} x"),
            ("simple_binomial_coefficient", r"\binom12"),
            ("stretchy_parentheses", r"\left( x \right)"),
            ("stretchy_one-sided_parenthesis", r"\left( x \right."),
            ("simple_integral", r"\int dx"),
            ("contour_integral", r"\oint_C dz"),
            ("simple_overset", r"\overset{n}{X}"),
            ("integral_with_limits", r"\int_0^1 dx"),
            ("integral_with_reversed_limits", r"\int^1_0 dx"),
            ("bold_font", r"\bm{x}"),
            ("black_board_font", r"\mathbb{R}"),
            ("sum_with_special_symbol", r"\sum_{i = 0}^∞ i"),
            ("product", r"\prod_n n"),
            ("underscore", r"x\ y"),
            ("stretchy_brace", r"\left\{ x  ( x + 2 ) \right\}"),
            ("prime", r"f'"),
            ("matrix", r"\begin{pmatrix} x \\ y \end{pmatrix}"),
            (
                "align",
                r#"\begin{align} f ( x ) &= x^2 + 2 x + 1 \\ &= ( x + 1 )^2\end{align}"#,
            ),
            (
                "text_transforms",
                r#"{fi}\ \mathit{fi}\ \mathrm{fi}\ \texttt{fi}"#,
            ),
            ("colon_fusion", r"a := 2 \land b :\equiv 3"),
            (
                "cases",
                r"f(x):=\begin{cases}0 &\text{if } x\geq 0\\1 &\text{otherwise}\end{cases}",
            ),
            ("mathstrut", r"\mathstrut"),
        ];

        for (name, problem) in problems.into_iter() {
            let alloc = Bump::new();
            let mathml = latex_to_mathml(&alloc, problem, crate::Display::Inline, true).unwrap();
            assert_snapshot!(name, &mathml, problem);
        }
    }

    #[test]
    fn error_test() {
        let problems = vec![
            ("end_without_open", r"\end{matrix}"),
            ("curly_close_without_open", r"}"),
            ("unsupported_command", r"\asdf"),
            (
                "unsupported_environment",
                r"\begin{xmatrix} 1 \end{xmatrix}",
            ),
            ("incorrect_bracket", r"\operatorname[lim}"),
            ("mismatched_begin_end", r"\begin{matrix} 1 \end{bmatrix}"),
            (
                "spaces_in_env_name",
                r"\begin{  pmatrix   } x \\ y \end{pmatrix}",
            ),
            (
                "incorrect_bracket_in_begin",
                r"\begin{matrix] 1 \end{matrix}",
            ),
            ("incomplete_sup", r"x^"),
            ("invalid_sup", r"x^^"),
            ("invalid_sub_sup", r"x^_"),
        ];

        for (name, problem) in problems.into_iter() {
            let alloc = &Bump::new();
            let results = get_nodes(alloc, problem);
            let error = format!("{:#?}", results.unwrap_err());
            assert_snapshot!(name, &error, problem);
        }
    }
}
