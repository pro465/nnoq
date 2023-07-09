use crate::error::{Error, ErrorTy, Loc};

pub struct Scanner<'a> {
    loc: Loc,
    peeked: Option<Result<Token, Error>>,
    rest: &'a str,
}

#[derive(Clone, Debug)]
pub struct Token {
    ty: TokenTy,
    loc: Loc,
}

impl Token {
    pub fn ty(self) -> TokenTy {
        self.ty
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenTy {
    Ident(String),
    Number(usize),
    Theorem,
    Type,
    Axiom,
    Colon,
    ColonEqual,
    DoubleColon,
    Comma,
    Lbrace,
    Rbrace,
    Lparen,
    Rparen,
    Lcurly,
    Rcurly,
    Eof,
}

impl<'a> Scanner<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            loc: Loc::new(),
            peeked: None,
            rest: s,
        }
    }

    pub fn expect_identifier(&mut self) -> Result<(Loc, String), Error> {
        let res = self.next_token()?;
        if let TokenTy::Ident(x) = res.ty {
            Ok((res.loc, x))
        } else {
            Err(Error {
                loc: res.loc,
                ty: ErrorTy::SyntaxError,
                desc: format!("expected idenifier, found token {:?}", res.ty),
            })
        }
    }

    pub fn expect_number(&mut self) -> Result<(Loc, usize), Error> {
        let res = self.next_token()?;
        if let TokenTy::Number(x) = res.ty {
            Ok((res.loc, x))
        } else {
            Err(Error {
                loc: self.loc,
                ty: ErrorTy::SyntaxError,
                desc: format!("expected number, found token {:?}", res.ty),
            })
        }
    }

    pub fn expect_one(&mut self, t: &[TokenTy]) -> Result<Token, Error> {
        let res = self.next_token()?;
        if !t.contains(&res.ty) {
            Err(Error {
                loc: res.loc,
                ty: ErrorTy::SyntaxError,
                desc: format!("expected one of {:?}, found token {:?}", t, res.ty),
            })
        } else {
            Ok(res)
        }
    }

    pub fn expect_token(&mut self, token: TokenTy) -> Result<Token, Error> {
        let res = self.next_token()?;
        if res.ty != token {
            Err(Error {
                loc: res.loc,
                ty: ErrorTy::SyntaxError,
                desc: format!("expected token {:?}, found token {:?}", token, res.ty),
            })
        } else {
            Ok(res)
        }
    }

    pub fn loc(&self) -> Loc {
        self.loc.clone()
    }

    pub fn is_token(&mut self, tok: TokenTy) -> Result<bool, Error> {
        if self.peek()?.ty == tok {
            self.expect_token(tok)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.peeked
            .take()
            .unwrap_or_else(|| self.next_token_internal())
    }

    pub fn peek(&mut self) -> Result<Token, Error> {
        let r = self.next_token();
        self.peeked = Some(r.clone());
        r
    }

    fn next_token_internal(&mut self) -> Result<Token, Error> {
        self.skip_whitespace();

        if self.rest.is_empty() {
            return Ok(Token {
                loc: self.loc(),
                ty: TokenTy::Eof,
            });
        }
        let mut iter = self.rest.char_indices();
        let (_, c) = iter.next().unwrap();

        if is_break(c) {
            use TokenTy::*;

            let ret = Ok(Token {
                loc: self.loc(),
                ty: match c {
                    ',' => Comma,
                    ':' => match iter.next() {
                        Some((_, ':')) => {
                            self.skip(1);
                            DoubleColon
                        }
                        Some((_, '=')) => {
                            self.skip(1);
                            ColonEqual
                        }
                        _ => Colon,
                    },
                    '[' => Lbrace,
                    ']' => Rbrace,
                    '(' => Lparen,
                    ')' => Rparen,
                    '{' => Lcurly,
                    '}' => Rcurly,
                    _ => {
                        return Err(Error {
                            loc: self.loc(),
                            ty: ErrorTy::SyntaxError,
                            desc: format!("unrecognized character {}", c),
                        })
                    }
                },
            });
            self.skip(c.len_utf8());
            ret
        } else {
            let mut i = self.rest.len();
            for (j, c) in iter {
                if is_break(c) {
                    i = j;
                    break;
                }
            }
            if c.is_alphabetic() {
                Ok(Token {
                    loc: self.loc(),
                    ty: self.ident_or_keyword(i),
                })
            } else if c.is_digit(10) {
                Ok(Token {
                    loc: self.loc(),
                    ty: self.parse_int(i)?,
                })
            } else {
                unreachable!()
            }
        }
    }

    fn ident_or_keyword(&mut self, i: usize) -> TokenTy {
        use TokenTy::*;
        let ident = &self.rest[..i];
        self.skip(i);
        match ident {
            "theorem" => Theorem,
            "axiom" => Axiom,
            "type" => Type,
            x => Ident(x.to_string()),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let i = self
                .rest
                .char_indices()
                .find(|(_i, c)| !c.is_whitespace())
                .map(|(i, _c)| i)
                .unwrap_or(self.rest.len());
            self.skip(i);
            if self.rest.chars().next() != Some('#') {
                break;
            }
            let i = self
                .rest
                .char_indices()
                .find(|(_i, c)| *c == '\n')
                .map(|(i, _c)| i + 1)
                .unwrap_or(self.rest.len());
            self.skip(i);
        }
    }

    fn parse_int(&mut self, i: usize) -> Result<TokenTy, Error> {
        let ident = &self.rest[..i].parse();
        match ident {
            Ok(x) => {
                self.skip(i);

                Ok(TokenTy::Number(*x))
            }
            Err(_) => Err(Error {
                loc: self.loc(),
                ty: ErrorTy::SyntaxError,
                desc: "failed to parse number".into(),
            }),
        }
    }

    fn skip(&mut self, len: usize) {
        for c in self.rest[..len].chars() {
            self.loc.col();
            if c == '\n' {
                self.loc.new_line();
            }
        }
        self.rest = &self.rest[len..];
    }
}

fn is_break(c: char) -> bool {
    !c.is_uppercase() && !c.is_lowercase() && !c.is_digit(10) && c != '_'
}
