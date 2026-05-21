use super::token::*;
use logos::{Lexer, Logos, Skip};

#[derive(Default, Clone, Copy)]
struct Position {
    row: usize,
    line_start: usize,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(extras = Position)]
enum LexTok {
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Identifier,
    #[regex(r"[0-9]+")]
    IntLiteral,
    #[regex(r"//[^\r\n]*", logos::skip, allow_greedy = true)]
    Comment,
    #[regex(r"==|!=|>=|<=|&&|\|\||[%;{}\[\]\(\)=><!\+\-\*/\,]")]
    Punctuation,
    #[regex(r"\r\n|\n|\r", logos::skip)]
    Whitespace,
    #[regex(r"[ \t]+", newline_callback)]
    Newline,
    #[regex(r"(?s:.)", |lex | lex.slice().chars().next().unwrap(), priority = 0)]
    //priority set to the lowest, so it's the last possible option
    Unknown(char),
}

//update line count and character index
fn newline_callback(lex: &mut Lexer<LexTok>) -> Skip {
    lex.extras.row += 1;
    lex.extras.line_start = lex.span().end;
    Skip
}

pub fn lex(src: &str) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();
    let mut lex = LexTok::lexer(src);

    while let Some(res) = lex.next() {
        let tok = match res {
            Ok(tok) => tok,
            Err(()) => unreachable!(
                "Error while lexing: Something went very, very wrong. Token completely skipped Unknown tokens."
            ),
        };

        let slice = lex.slice();
        let span = lex.span();
        let (tag, lexeme) = match tok {
            LexTok::Identifier => {
                let tag = keyword_or_identifier(slice);
                let lx = if tag == TokenTag::NAME {
                    Some(TokenLexeme::Name(slice.to_string()))
                } else {
                    None
                };
                (tag, lx)
            }
            LexTok::IntLiteral => (
                TokenTag::INTLIT,
                Some(TokenLexeme::Int(slice.parse::<i64>().unwrap())),
            ),

            LexTok::Punctuation => {
                let tag = punctuation(slice);
                (tag, None)
            }

            LexTok::Comment | LexTok::Whitespace | LexTok::Newline => continue,
            LexTok::Unknown(ch) => (TokenTag::UNKNOWN, Some(TokenLexeme::Unknown(ch))),
        };

        let (row, col) = (
            lex.extras.row as u32,
            (span.start - lex.extras.line_start) as u32,
        );

        out.push(Token::new(tag, span.start, span.end, row, col, lexeme));
    }
    out
}

fn punctuation(s: &str) -> TokenTag {
    match s {
        //punctuation / delimitiers
        ";" => TokenTag::SEMICOLON, //   ;
        "," => TokenTag::COMMA,     //   ,
        "{" => TokenTag::LCURLY,    //   {
        "}" => TokenTag::RCURLY,    //   }
        "(" => TokenTag::LPAREN,    //   (
        ")" => TokenTag::RPAREN,    //   )
        "[" => TokenTag::LSQUARE,   //   [
        "]" => TokenTag::RSQUARE,   //   ]

        //operators
        "=" => TokenTag::EQ,     //   =
        "==" => TokenTag::EQEQ,  //   ==
        "!=" => TokenTag::NOTEQ, //   !=
        ">=" => TokenTag::GEQ,   //   >=
        "<=" => TokenTag::LEQ,   //   <=
        ">" => TokenTag::GT,     //   >
        "<" => TokenTag::LT,     //   <
        "&&" => TokenTag::AND,   //   &&
        "||" => TokenTag::OR,    //   ||
        "!" => TokenTag::NOT,    //   !
        "+" => TokenTag::PLUS,   //   +
        "-" => TokenTag::MINUS,  //   -
        "*" => TokenTag::TIMES,  //   *
        "/" => TokenTag::DIVIDE, //   /
        "%" => TokenTag::MOD,    //   %
        _ => TokenTag::UNKNOWN,
    }
}

fn keyword_or_identifier(s: &str) -> TokenTag {
    match s {
        //literals and identifiers
        //"" => TokenTag::INTLIT,

        //"" => TokenTag::EOF,
        //type keywords
        "bool" => TokenTag::BOOL,   // bool
        "int" => TokenTag::INT,     // int
        "unit" => TokenTag::UNIT,   // unit
        "true" => TokenTag::TRUE,   // true (lowercase)
        "false" => TokenTag::FALSE, // false (lowercase )
        //keywords
        "continue" => TokenTag::CONTINUE, //    continue
        "if" => TokenTag::IF,             //    if
        "else" => TokenTag::ELSE,         //    else
        "while" => TokenTag::WHILE,       //    while
        "for" => TokenTag::FOR,           //    for
        "break" => TokenTag::BREAK,       //    break
        "return" => TokenTag::RETURN,
        "main" => TokenTag::MAIN,
        _ => TokenTag::NAME,
    }
}
