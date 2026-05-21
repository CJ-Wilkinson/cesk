use std::fmt;
use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TokenTag {
    //literals and identifiers
    INTLIT,
    NAME,
    UNKNOWN,
    EOF,

    //punctuation / delimitiers
    SEMICOLON, //   ;
    COMMA,     //   ,

    LCURLY,  //   {
    RCURLY,  //   }
    LPAREN,  //   (
    RPAREN,  //   )
    LSQUARE, //   [
    RSQUARE, //   ]

    //operators
    EQ,     //   =
    EQEQ,   //   ==
    NOTEQ,  //   !=
    GEQ,    //   >=
    LEQ,    //   <=
    GT,     //   >
    LT,     //   <
    AND,    //   &&
    OR,     //   ||
    NOT,    //   !
    PLUS,   //   +
    MINUS,  //   -
    TIMES,  //   *
    DIVIDE, //   /
    MOD,    //   %

    //type keywords
    BOOL,  // bool
    INT,   // int
    UNIT,  // unit
    TRUE,  // true (lowercase)
    FALSE, // false (lowercase )

    //keywords
    CONTINUE, //    continue
    IF,       //    if
    ELSE,     //    else
    WHILE,    //    while
    FOR,      //    for
    BREAK,    //    break
    RETURN,
}

impl Display for TokenTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSpan {
    //    ! do we actually need this?
    //byte offsets (for debugging)
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenLocation {
    pub row: u32,
    pub column: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenLexeme {
    Int(i64),
    Name(String),
    Unknown(char),
}

impl Display for TokenLexeme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenLexeme::Int(n) => write!(f, "{n}"),
            TokenLexeme::Name(name) => write!(f, "{name}"),
            TokenLexeme::Unknown(c) => write!(f, "{c}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    tag: TokenTag,
    span: TokenSpan,
    location: TokenLocation,
    lexeme: Option<TokenLexeme>,
}

impl Token {
    pub fn new(
        tag: TokenTag,
        start: usize,
        end: usize,
        row: u32,
        column: u32,
        lexeme: Option<TokenLexeme>,
    ) -> Self {
        Self {
            tag,
            span: TokenSpan { start, end },
            location: TokenLocation { row, column },
            lexeme,
        }
    }

    pub fn kind(&self) -> TokenTag {
        self.tag
    }

    pub fn location(&self) -> TokenLocation {
        self.location
    }

    pub fn row(&self) -> u32 {
        self.location.row
    }

    pub fn column(&self) -> u32 {
        self.location.column
    }
    pub fn span(&self) -> TokenSpan {
        self.span
    }

    pub fn start(&self) -> usize {
        self.span.start
    }

    pub fn end(&self) -> usize {
        self.span.end
    }

    pub fn lexeme(&self) -> Option<&TokenLexeme> {
        self.lexeme.as_ref()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = format!("{}", self.tag);

        let lex = self
            .lexeme
            .as_ref()
            .map_or("NONE".to_string(), |lx| lx.to_string());

        write!(f, "{: <20} | {:<4} | {}", tag, self.location.row, lex)
    }
}
