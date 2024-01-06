use strum_macros::AsRefStr;

use crate::attribute::{DisplayStyle, TextTransform};
use crate::ops::{self, Op};

#[derive(Debug, Clone, PartialEq, AsRefStr)]
pub enum Token<'a> {
    Null,
    #[strum(serialize = "end of document")]
    EOF,
    #[strum(serialize = "\\begin{...}")]
    Begin,
    #[strum(serialize = "\\end{...}")]
    End,
    #[strum(serialize = "&")]
    Ampersand,
    NewLine,
    Left,
    Right,
    Middle,
    Paren(Op),
    #[strum(serialize = "{")]
    LBrace,
    #[strum(serialize = "}")]
    RBrace,
    Frac(Option<DisplayStyle>),
    Underscore,
    Circumflex,
    Binom(Option<DisplayStyle>),
    Overset,
    Underset,
    Overbrace(Op),
    Underbrace(Op),
    #[strum(serialize = "\\sqrt")]
    Sqrt,
    Integral(Op),
    Lim(&'static str),
    Space(&'static str),
    NonBreakingSpace,
    Style(TextTransform),
    NormalVariant,
    Big(&'static str),
    Over(Op),
    Under(Op),
    Operator(Op),
    #[strum(serialize = ">")]
    OpGreaterThan,
    #[strum(serialize = "<")]
    OpLessThan,
    #[strum(serialize = ":")]
    Colon,
    BigOp(Op),
    Letter(char),
    NormalLetter(char),
    Number(String, Op),
    Function(&'static str),
    OperatorName,
    Slashed,
    Text,
    Mathstrut,
    UnknownCommand(&'a str),
}

impl<'a> Token<'a> {
    pub(crate) fn acts_on_a_digit(&self) -> bool {
        matches!(
            self,
            Token::Sqrt | Token::Frac(_) | Token::Binom(_) | Token::Style(_)
        )
    }

    pub fn from_command(command: &'a str) -> Token<'a> {
        match command {
            "mathrm" => Token::NormalVariant,
            "textit" => Token::Style(TextTransform::Italic),
            "mathit" => Token::Style(TextTransform::Italic),
            "mathcal" => Token::Style(TextTransform::Script),
            "textbf" => Token::Style(TextTransform::Bold),
            "mathbf" => Token::Style(TextTransform::Bold),
            "bm" => Token::Style(TextTransform::BoldItalic),
            "symbf" => Token::Style(TextTransform::BoldItalic),
            "mathbb" => Token::Style(TextTransform::DoubleStruck),
            "mathfrak" => Token::Style(TextTransform::Fraktur),
            "mathscr" => Token::Style(TextTransform::Script),
            "mathsf" => Token::Style(TextTransform::SansSerif),
            "texttt" => Token::Style(TextTransform::Monospace),
            "boldsymbol" => Token::Style(TextTransform::BoldItalic),
            "mathstrut" => Token::Mathstrut,
            "text" => Token::Text,
            "sqrt" => Token::Sqrt,
            "frac" => Token::Frac(None),
            "tfrac" => Token::Frac(Some(DisplayStyle::False)),
            "dfrac" => Token::Frac(Some(DisplayStyle::True)),
            "left" => Token::Left,
            "right" => Token::Right,
            "middle" => Token::Middle,
            "begin" => Token::Begin,
            "end" => Token::End,
            "\\" => Token::NewLine,
            "binom" => Token::Binom(None),
            "tbinom" => Token::Binom(Some(DisplayStyle::False)),
            "dbinom" => Token::Binom(Some(DisplayStyle::True)),
            "overset" => Token::Overset,
            "underset" => Token::Underset,
            "overbrace" => Token::Overbrace(Op('\u{23de}')),
            "underbrace" => Token::Underbrace(Op('\u{23df}')),
            "overparen" => Token::Overbrace(Op('\u{23dc}')),
            "underparen" => Token::Underbrace(Op('\u{23dd}')),
            "overbracket" => Token::Overbrace(Op('\u{23b4}')),
            "underbracket" => Token::Underbrace(Op('\u{23b5}')),
            "!" => Token::Space("-0.1667"),
            "," => Token::Space("0.1667"),
            ":" => Token::Space("0.2222"),
            ";" => Token::Space("0.2778"),
            " " => Token::Space("1"),
            "quad" => Token::Space("1"),
            "qquad" => Token::Space("2"),
            "langle" => Token::Paren(Op('〈')),
            "rangle" => Token::Paren(Op('〉')),
            "{" => Token::Paren(ops::LEFT_CURLY_BRACKET),
            "}" => Token::Paren(ops::RIGHT_CURLY_BRACKET),
            "lceil" => Token::Paren(Op('⌈')),
            "rceil" => Token::Paren(Op('⌉')),
            "lfloor" => Token::Paren(Op('⌊')),
            "rfloor" => Token::Paren(Op('⌋')),
            "lgroup" => Token::Paren(Op('⦗')),
            "rgroup" => Token::Paren(Op('⦘')),
            "llbracket" => Token::Paren(Op('⟦')),
            "rrbracket" => Token::Paren(Op('⟧')),
            "lim" => Token::Lim("lim"),
            "liminf" => Token::Lim("lim inf"),
            "limsup" => Token::Lim("lim sup"),
            "min" => Token::Lim("min"),
            "max" => Token::Lim("max"),
            "inf" => Token::Lim("inf"),
            "sup" => Token::Lim("sup"),
            "int" => Token::Integral(Op('∫')),
            "iint" => Token::Integral(Op('∬')),
            "iiint" => Token::Integral(Op('∭')),
            "oint" => Token::Integral(Op('∮')),
            "dot" => Token::Over(Op('\u{02d9}')),
            "ddot" => Token::Over(Op('¨')),
            "bar" => Token::Over(Op('¯')),
            "hat" => Token::Over(Op('^')),
            "check" => Token::Over(Op('ˇ')),
            "breve" => Token::Over(Op('˘')),
            "acute" => Token::Over(Op('´')),
            "grave" => Token::Over(Op('`')),
            "tilde" => Token::Over(Op('~')),
            "vec" => Token::Over(Op('→')),
            "overline" => Token::Over(Op('_')),
            "underline" => Token::Under(Op('_')),
            "widehat" => Token::Over(Op('^')),
            "widetilde" => Token::Over(Op('~')),
            "overrightarrow" => Token::Over(Op('→')),
            "overleftarrow" => Token::Over(Op('←')),
            "sum" => Token::BigOp(ops::SUM),
            "prod" => Token::BigOp(ops::PROD),
            "coprod" => Token::BigOp(Op('∐')),
            "bigcap" => Token::BigOp(Op('⋂')),
            "bigcup" => Token::BigOp(Op('⋃')),
            "bigsqcup" => Token::BigOp(Op('⨆')),
            "bigvee" => Token::BigOp(Op('⋁')),
            "bigwedge" => Token::BigOp(Op('⋀')),
            "bigodot" => Token::BigOp(Op('⨀')),
            "bitotimes" => Token::BigOp(Op('⨂')),
            "bigoplus" => Token::BigOp(Op('⨁')),
            "biguplus" => Token::BigOp(Op('⨄')),
            "bigl" => Token::Big("1.2em"),
            "bigr" => Token::Big("1.2em"),
            "Bigl" => Token::Big("1.623em"),
            "Bigr" => Token::Big("1.623em"),
            "biggl" => Token::Big("2.047em"),
            "biggr" => Token::Big("2.047em"),
            "Biggl" => Token::Big("2.470em"),
            "Biggr" => Token::Big("2.470em"),
            // <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
            //   <semantics>
            //     <mrow>
            //         <mi>a</mi>
            //         <mrow class="MJX-TeXAtom-OPEN"><mo maxsize="2.470em" minsize="2.470em">(</mo></ mrow>
            //         <mi>b</mi>
            //         <mrow class="MJX-TeXAtom-OPEN"><mo maxsize="2.047em" minsize="2.047em">(</mo></mrow>
            //         <mi>c</mi>
            //         <mrow class="MJX-TeXAtom-OPEN"><mo maxsize="1.623em" minsize="1.623em">(</mo></mrow>
            //         <mi>d</mi>
            //         <mrow class="MJX-TeXAtom-OPEN"><mo maxsize="1.2em" minsize="1.2em">(</mo></mrow>
            //         <mi>e</mi>
            //         <mo stretchy="false">(</mo>
            //         <mi>f</mi>
            //         <mo>+</mo>
            //         <mi>g</mi>
            //         <mo stretchy="false">)</mo>
            //         <mrow class="MJX-TeXAtom-CLOSE"><mo maxsize="1.2em" minsize="1.2em">)</mo></mrow>
            //         <mrow class="MJX-TeXAtom-CLOSE"><mo maxsize="1.623em" minsize="1.623em">)</mo></mrow>
            //         <mrow class="MJX-TeXAtom-CLOSE"><mo maxsize="2.047em" minsize="2.047em">)</mo></mrow>
            //         <mrow class="MJX-TeXAtom-CLOSE"><mo maxsize="2.470em" minsize="2.470em">)</mo></mrow>
            //   </semantics>
            // </math>
            "sin" => Token::Function("sin"),
            "cos" => Token::Function("cos"),
            "tan" => Token::Function("tan"),
            "csc" => Token::Function("csc"),
            "sec" => Token::Function("sec"),
            "cot" => Token::Function("cot"),
            "arcsin" => Token::Function("arcsin"),
            "arccos" => Token::Function("arccos"),
            "arctan" => Token::Function("arctan"),
            "sinh" => Token::Function("sinh"),
            "cosh" => Token::Function("cosh"),
            "tanh" => Token::Function("tanh"),
            "coth" => Token::Function("coth"),
            "exp" => Token::Function("exp"),
            "ln" => Token::Function("ln"),
            "log" => Token::Function("log"),
            "erf" => Token::Function("erf"),
            "erfc" => Token::Function("erfc"),
            "arg" => Token::Function("arg"),
            "ker" => Token::Function("ker"),
            "dim" => Token::Function("dim"),
            "det" => Token::Function("det"),
            "wp" => Token::Function("℘"),
            "operatorname" => Token::OperatorName,
            "Alpha" => Token::NormalLetter('Α'),
            "alpha" => Token::Letter('α'),
            "Beta" => Token::NormalLetter('Β'),
            "beta" => Token::Letter('β'),
            "Gamma" => Token::NormalLetter('Γ'),
            "gamma" => Token::Letter('γ'),
            "digamma" => Token::Letter('ϝ'),
            "Delta" => Token::NormalLetter('Δ'),
            "delta" => Token::Letter('δ'),
            "Epsilon" => Token::NormalLetter('Ε'),
            "epsilon" => Token::Letter('ϵ'),
            "varepsilon" => Token::Letter('ε'),
            "Zeta" => Token::NormalLetter('Ζ'),
            "zeta" => Token::Letter('ζ'),
            "Eta" => Token::NormalLetter('Η'),
            "eta" => Token::Letter('η'),
            "Theta" => Token::NormalLetter('Θ'),
            "theta" => Token::Letter('θ'),
            "vartheta" => Token::Letter('ϑ'),
            "Iota" => Token::NormalLetter('Ι'),
            "iota" => Token::Letter('ι'),
            "Kappa" => Token::NormalLetter('Κ'),
            "kappa" => Token::Letter('κ'),
            "Lambda" => Token::NormalLetter('Λ'),
            "lambda" => Token::Letter('λ'),
            "Mu" => Token::NormalLetter('Μ'),
            "mu" => Token::Letter('μ'),
            "Nu" => Token::NormalLetter('Ν'),
            "nu" => Token::Letter('ν'),
            "Xi" => Token::NormalLetter('Ξ'),
            "xi" => Token::Letter('ξ'),
            "Omicron" => Token::NormalLetter('Ο'),
            "omicron" => Token::Letter('ο'),
            "Pi" => Token::NormalLetter('Π'),
            "pi" => Token::Letter('π'),
            "varpi" => Token::Letter('ϖ'),
            "Rho" => Token::NormalLetter('Ρ'),
            "rho" => Token::Letter('ρ'),
            "varrho" => Token::Letter('ϱ'),
            "Sigma" => Token::NormalLetter('Σ'),
            "sigma" => Token::Letter('σ'),
            "varsigma" => Token::Letter('ς'),
            "Tau" => Token::NormalLetter('Τ'),
            "tau" => Token::Letter('τ'),
            "Upsilon" => Token::NormalLetter('Υ'),
            "upsilon" => Token::Letter('υ'),
            "Phi" => Token::NormalLetter('Φ'),
            "phi" => Token::Letter('ϕ'),
            "varphi" => Token::Letter('φ'),
            "Chi" => Token::NormalLetter('Χ'),
            "chi" => Token::Letter('χ'),
            "Psi" => Token::NormalLetter('Ψ'),
            "psi" => Token::Letter('ψ'),
            "Omega" => Token::NormalLetter('Ω'),
            "omega" => Token::Letter('ω'),
            "aleph" => Token::NormalLetter('ℵ'),
            "beth" => Token::NormalLetter('ℶ'),
            "gimel" => Token::NormalLetter('ℷ'),
            "daleth" => Token::NormalLetter('ℸ'),
            "A" => Token::NormalLetter('Å'),
            "a" => Token::NormalLetter('å'),
            "AE" => Token::NormalLetter('Æ'),
            "ae" => Token::NormalLetter('æ'),
            "DH" => Token::NormalLetter('Ð'),
            "dh" => Token::NormalLetter('ð'),
            "dj" => Token::NormalLetter('đ'),
            "L" => Token::NormalLetter('Ł'),
            "l" => Token::NormalLetter('ł'),
            "NG" => Token::NormalLetter('Ŋ'),
            "ng" => Token::NormalLetter('ŋ'),
            "O" => Token::NormalLetter('Ø'),
            "o" => Token::NormalLetter('ø'),
            "OE" => Token::NormalLetter('Œ'),
            "oe" => Token::NormalLetter('œ'),
            "ss" => Token::NormalLetter('ß'),
            "TH" => Token::NormalLetter('Þ'),
            "th" => Token::NormalLetter('þ'),
            "imath" => Token::Letter('ı'),
            "jmath" => Token::Letter('ȷ'),
            "ell" => Token::Letter('ℓ'),
            "hbar" => Token::Letter('ℏ'),
            "hslash" => Token::Letter('ℏ'),
            "infty" => Token::NormalLetter('∞'),
            "mho" => Token::NormalLetter('℧'),
            "Finv" => Token::NormalLetter('Ⅎ'),
            "Re" => Token::NormalLetter('ℜ'),
            "Im" => Token::NormalLetter('ℑ'),
            "complement" => Token::NormalLetter('∁'),
            "emptyset" => Token::NormalLetter('∅'),
            "varnothing" => Token::Letter('⌀'),
            "therefore" => Token::NormalLetter('∴'),
            "because" => Token::NormalLetter('∵'),
            "Diamond" => Token::NormalLetter('◊'),
            "Box" => Token::NormalLetter('◻'),
            "triangle" => Token::NormalLetter('△'),
            "angle" => Token::NormalLetter('∠'),
            "dagger" => Token::NormalLetter('†'),
            "dag" => Token::NormalLetter('†'),
            "Dagger" => Token::NormalLetter('‡'),
            "ddag" => Token::NormalLetter('‡'),
            "And" => Token::NormalLetter('&'),
            "eth" => Token::NormalLetter('ð'),
            "S" => Token::NormalLetter('§'),
            "P" => Token::NormalLetter('¶'),
            "%" => Token::NormalLetter('%'),
            "_" => Token::NormalLetter('_'),
            "&" => Token::NormalLetter('&'),
            "#" => Token::NormalLetter('#'),
            "$" => Token::NormalLetter('$'),
            "copyright" => Token::NormalLetter('©'),
            "checkmark" => Token::NormalLetter('✓'),
            "circledR" => Token::NormalLetter('Ⓡ'),
            "maltese" => Token::NormalLetter('✠'),
            "colon" => Token::NormalLetter(':'),
            "bigtriangleup" => Token::NormalLetter('△'),
            "sphericalangle" => Token::NormalLetter('∢'),
            "square" => Token::NormalLetter('□'),
            "lozenge" => Token::NormalLetter('◊'),
            "diamondsuit" => Token::NormalLetter('♢'),
            "heartsuit" => Token::NormalLetter('♡'),
            "clubsuit" => Token::NormalLetter('♣'),
            "spadesuit" => Token::NormalLetter('♠'),
            "Game" => Token::NormalLetter('⅁'),
            "flat" => Token::NormalLetter('♭'),
            "natural" => Token::NormalLetter('♮'),
            "sharp" => Token::NormalLetter('♯'),
            "pounds" => Token::NormalLetter('£'),
            "textyen" => Token::NormalLetter('¥'),
            "euro" => Token::NormalLetter('€'),
            "rupee" => Token::NormalLetter('₹'),
            "sun" => Token::NormalLetter('☼'),
            "mercury" => Token::NormalLetter('☿'),
            "venus" => Token::NormalLetter('♀'),
            "earth" => Token::NormalLetter('♁'),
            "mars" => Token::NormalLetter('♂'),
            "jupiter" => Token::NormalLetter('♃'),
            "saturn" => Token::NormalLetter('♄'),
            "uranus" => Token::NormalLetter('♅'),
            "neptune" => Token::NormalLetter('♆'),
            "astrosun" => Token::NormalLetter('☉'),
            "ascnode" => Token::NormalLetter('☊'),
            "times" => Token::Operator(ops::TIMES),
            "oplus" => Token::Operator(Op('⊕')),
            "ominus" => Token::Operator(Op('⊖')),
            "otimes" => Token::Operator(Op('⊗')),
            "oslash" => Token::Operator(Op('⊘')),
            "odot" => Token::Operator(Op('⊙')),
            "bigcirc" => Token::Operator(Op('◯')),
            "amalg" => Token::Operator(Op('⨿')),
            "pm" => Token::Operator(Op('±')),
            "mp" => Token::Operator(Op('∓')),
            "cdot" => Token::Operator(Op('·')),
            "dots" => Token::Operator(Op('…')),
            "cdots" => Token::Operator(Op('⋯')),
            "vdots" => Token::Operator(Op('⋮')),
            "ldots" => Token::Operator(Op('…')),
            "ddots" => Token::Operator(Op('⋱')),
            "circ" => Token::Operator(Op('∘')),
            "bullet" => Token::Operator(Op('∙')),
            "star" => Token::Operator(Op('⋆')),
            "div" => Token::Operator(Op('÷')),
            "lnot" => Token::Operator(Op('¬')),
            "neg" => Token::Operator(Op('¬')),
            "land" => Token::Operator(Op('∧')),
            "lor" => Token::Operator(Op('∨')),
            "sim" => Token::Operator(Op('∼')),
            "simeq" => Token::Operator(Op('≃')),
            "nsim" => Token::Operator(Op('≁')),
            "cong" => Token::Operator(Op('≅')),
            "approx" => Token::Operator(Op('≈')),
            "ne" => Token::Operator(Op('≠')),
            "neq" => Token::Operator(Op('≠')),
            "equiv" => Token::Operator(Op('≡')),
            "nequiv" => Token::Operator(Op('≢')),
            "prec" => Token::Operator(Op('≺')),
            "succ" => Token::Operator(Op('≻')),
            "preceq" => Token::Operator(Op('⪯')),
            "succeq" => Token::Operator(Op('⪰')),
            "dashv" => Token::Operator(Op('⊣')),
            "asymp" => Token::Operator(Op('≍')),
            "doteq" => Token::Operator(Op('≐')),
            "propto" => Token::Operator(Op('∝')),
            "barwedge" => Token::Operator(Op('⊼')),
            "ltimes" => Token::Operator(Op('⋉')),
            "rtimes" => Token::Operator(Op('⋊')),
            "Join" => Token::Operator(Op('⋈')),
            "lhd" => Token::Operator(Op('⊲')),
            "rhd" => Token::Operator(Op('⊳')),
            "unlhd" => Token::Operator(Op('⊴')),
            "unrhd" => Token::Operator(Op('⊵')),
            "vee" => Token::Operator(Op('∨')),
            "uplus" => Token::Operator(Op('⊎')),
            "wedge" => Token::Operator(Op('∧')),
            "boxdot" => Token::Operator(Op('⊡')),
            "boxplus" => Token::Operator(Op('⊞')),
            "boxminus" => Token::Operator(Op('⊟')),
            "boxtimes" => Token::Operator(Op('⊠')),
            "boxbox" => Token::Operator(Op('⧈')),
            "boxslash" => Token::Operator(Op('⧄')),
            "boxbslash" => Token::Operator(Op('⧅')),
            "Cap" => Token::Operator(Op('⋒')),
            "Cup" => Token::Operator(Op('⋓')),
            "centerdot" => Token::Operator(Op('∙')),
            "circledast" => Token::Operator(Op('⊛')),
            "circledcirc" => Token::Operator(Op('⊚')),
            "circleddash" => Token::Operator(Op('⊝')),
            "curlyvee" => Token::Operator(Op('⋎')),
            "curlywedge" => Token::Operator(Op('⋏')),
            "dotplus" => Token::Operator(Op('∔')),
            "intercal" => Token::Operator(Op('⊺')),
            "divideontimes" => Token::Operator(Op('⋇')),
            "leftthreetimes" => Token::Operator(Op('⋋')),
            "rightthreetimes" => Token::Operator(Op('⋌')),
            "smallsetminus" => Token::Operator(Op('﹨')),
            "triangledown" => Token::Operator(Op('▽')),
            "triangleleft" => Token::Operator(Op('◁')),
            "triangleright" => Token::Operator(Op('▷')),
            "vartriangle" => Token::Operator(Op('△')),
            "veebar" => Token::Operator(Op('⊻')),
            "cap" => Token::Operator(Op('∩')),
            "cup" => Token::Operator(Op('∪')),
            "mid" => Token::Operator(Op('\u{2223}')),
            "nmid" => Token::Operator(Op('\u{2224}')),
            "|" => Token::Paren(Op('‖')),
            "parallel" => Token::Operator(Op('∥')),
            "perp" => Token::Operator(Op('⊥')),
            "forall" => Token::Operator(ops::FORALL),
            "exists" => Token::Operator(ops::EXISTS),
            "nexists" => Token::Operator(Op('∄')),
            "lt" => Token::OpLessThan,
            "gt" => Token::OpGreaterThan,
            "leq" => Token::Operator(Op('≤')),
            "geq" => Token::Operator(Op('≥')),
            "le" => Token::Operator(Op('≤')),
            "ge" => Token::Operator(Op('≥')),
            "ll" => Token::Operator(Op('≪')),
            "gg" => Token::Operator(Op('≫')),
            "lessapprox" => Token::Operator(Op('⪅')),
            "lesssim" => Token::Operator(Op('≲')),
            "eqslantless" => Token::Operator(Op('⪕')),
            "leqslant" => Token::Operator(Op('⩽')),
            "leqq" => Token::Operator(Op('≦')),
            "geqq" => Token::Operator(Op('≧')),
            "geqslant" => Token::Operator(Op('⩾')),
            "eqslantgtr" => Token::Operator(Op('⪖')),
            "gtrsim" => Token::Operator(Op('≳')),
            "gtrapprox" => Token::Operator(Op('⪆')),
            "approxeq" => Token::Operator(Op('≊')),
            "lessdot" => Token::Operator(Op('⋖')),
            "lll" => Token::Operator(Op('⋘')),
            "lessgtr" => Token::Operator(Op('≶')),
            "lesseqgtr" => Token::Operator(Op('⋚')),
            "lesseqqgtr" => Token::Operator(Op('⪋')),
            "doteqdot" => Token::Operator(Op('≑')),
            "risingdotseq" => Token::Operator(Op('≓')),
            "leftarrow" => Token::Operator(Op('←')),
            "gets" => Token::Operator(Op('←')),
            "rightarrow" => Token::Operator(Op('→')),
            "to" => Token::Operator(Op('→')),
            "nleftarrow" => Token::Operator(Op('↚')),
            "nrightarrow" => Token::Operator(Op('↛')),
            "leftrightarrow" => Token::Operator(Op('↔')),
            "nleftrightarrow" => Token::Operator(Op('↮')),
            "longleftarrow" => Token::Operator(Op('⟵')),
            "longrightarrow" => Token::Operator(Op('⟶')),
            "longleftrightarrow" => Token::Operator(Op('⟷')),
            "Leftarrow" => Token::Operator(Op('⇐')),
            "Rightarrow" => Token::Operator(Op('⇒')),
            "nLeftarrow" => Token::Operator(Op('⇍')),
            "nRightarrow" => Token::Operator(Op('⇏')),
            "Leftrightarrow" => Token::Operator(Op('⇔')),
            "nLeftrightarrow" => Token::Operator(Op('⇎')),
            "Longleftarrow" => Token::Operator(Op('⟸')),
            "impliedby" => Token::Operator(Op('⟸')),
            "Longrightarrow" => Token::Operator(Op('⟹')),
            "implies" => Token::Operator(Op('⟹')),
            "Longleftrightarrow" => Token::Operator(Op('⟺')),
            "iff" => Token::Operator(Op('⟺')),
            "uparrow" => Token::Operator(Op('↑')),
            "downarrow" => Token::Operator(Op('↓')),
            "updownarrow" => Token::Operator(Op('↕')),
            "Uparrow" => Token::Operator(Op('⇑')),
            "Downarrow" => Token::Operator(Op('⇓')),
            "Updownarrow" => Token::Operator(Op('⇕')),
            "nearrow" => Token::Operator(Op('↗')),
            "searrow" => Token::Operator(Op('↘')),
            "swarrow" => Token::Operator(Op('↙')),
            "nwarrow" => Token::Operator(Op('↖')),
            "rightharpoonup" => Token::Operator(Op('⇀')),
            "rightharpoondown" => Token::Operator(Op('⇁')),
            "leftharpoonup" => Token::Operator(Op('↼')),
            "leftharpoondown" => Token::Operator(Op('↽')),
            "upharpoonleft" => Token::Operator(Op('↿')),
            "upharpoonright" => Token::Operator(Op('↾')),
            "downharpoonleft" => Token::Operator(Op('⇃')),
            "downharpoonright" => Token::Operator(Op('⇂')),
            "rightleftharpoons" => Token::Operator(Op('⇌')),
            "leftrightharpoons" => Token::Operator(Op('⇋')),
            "curvearrowleft" => Token::Operator(Op('↶')),
            "circlearrowleft" => Token::Operator(Op('↺')),
            "Lsh" => Token::Operator(Op('↰')),
            "upuparrows" => Token::Operator(Op('⇈')),
            "rightrightarrows" => Token::Operator(Op('⇉')),
            "rightleftarrows" => Token::Operator(Op('⇄')),
            "Rrightarrow" => Token::Operator(Op('⇛')),
            "rightarrowtail" => Token::Operator(Op('↣')),
            "looparrowright" => Token::Operator(Op('↬')),
            "curvearrowright" => Token::Operator(Op('↷')),
            "circlearrowright" => Token::Operator(Op('↻')),
            "Rsh" => Token::Operator(Op('↱')),
            "downdownarrows" => Token::Operator(Op('⇊')),
            "leftleftarrows" => Token::Operator(Op('⇇')),
            "leftrightarrows" => Token::Operator(Op('⇆')),
            "Lleftarrow" => Token::Operator(Op('⇚')),
            "leftarrowtail" => Token::Operator(Op('↢')),
            "looparrowleft" => Token::Operator(Op('↫')),
            "mapsto" => Token::Operator(Op('↦')),
            "longmapsto" => Token::Operator(Op('⟼')),
            "hookrightarrow" => Token::Operator(Op('↪')),
            "hookleftarrow" => Token::Operator(Op('↩')),
            "multimap" => Token::Operator(Op('⊸')),
            "leftrightsquigarrow" => Token::Operator(Op('↭')),
            "rightsquigarrow" => Token::Operator(Op('⇝')),
            "lightning" => Token::Operator(Op('↯')),
            "Yleft" => Token::Operator(Op('⤙')),
            "Yright" => Token::Operator(Op('⤚')),
            "in" => Token::Operator(ops::ISIN),
            "ni" => Token::Operator(ops::NI),
            "notin" => Token::Operator(ops::NOTIN),
            "subset" => Token::Operator(Op('\u{2282}')),
            "supset" => Token::Operator(Op('\u{2283}')),
            "subseteq" => Token::Operator(Op('\u{2286}')),
            "supseteq" => Token::Operator(Op('\u{2287}')),
            "nsubseteq" => Token::Operator(Op('\u{2288}')),
            "nsupseteq" => Token::Operator(Op('\u{2289}')),
            "subsetneq" => Token::Operator(Op('\u{228a}')),
            "supsetneq" => Token::Operator(Op('\u{228b}')),
            "sqsubset" => Token::Operator(Op('⊏')),
            "sqsubseteq" => Token::Operator(Op('⊑')),
            "sqsupset" => Token::Operator(Op('⊐')),
            "sqsupseteq" => Token::Operator(Op('⊒')),
            "sqcap" => Token::Operator(Op('⊓')),
            "sqcup" => Token::Operator(Op('⊔')),
            "setminus" => Token::Operator(Op('∖')),
            "partial" => Token::Letter('∂'),
            "nabla" => Token::Operator(Op('∇')),
            "smile" => Token::Operator(Op('⌣')),
            "from" => Token::Operator(Op('⌢')),
            "wr" => Token::Operator(Op('≀')),
            "bot" => Token::Operator(Op('⊥')),
            "top" => Token::Operator(Op('⊤')),
            "vdash" => Token::Operator(Op('⊢')),
            "vDash" => Token::Operator(Op('⊨')),
            "Vdash" => Token::Operator(Op('⊩')),
            "models" => Token::Operator(Op('⊨')),
            "slashed" => Token::Slashed,
            _ => Token::UnknownCommand(command),
        }
    }
}
