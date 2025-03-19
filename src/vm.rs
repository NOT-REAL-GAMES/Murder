use crate::core::*;

/// Represent any valid JSON value.
#[derive(Debug)]
enum Value {
    /// null.
    Null,
    /// true or false.
    Bool(bool),
    /// Any floating point number.
    Number(f64),
    /// Any quoted string.
    String(String),
    /// An array of values
    Array(Vec<Value>),
    /// An dictionary mapping keys and values.
    Object(HashMap<String, Value>),
}

#[derive(Logos, Debug)]
enum Token {
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,
    #[token("=")]
    AssignEqual,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(".")]
    Period,
    #[token("null")]
    Null,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token("if")]
    KeywordIf,
    #[regex(r#"\"([a-zA-Z,.:;!\[\]\{\} ]*)\""#, |x| x.slice().to_owned(), priority = 1)]
    String(String),
    #[regex(r#"([a-zA-Z]*)"#, |x| x.slice().to_owned(), priority = 1)]
    Text(String),
    #[regex(r"[\s\t\n]*")]
    Whitespace
}

pub fn run_vm(
    mut w: &mut World
){        
    unsafe{

        let cell = w.as_unsafe_world_cell();
    
        let wrld = cell.world();
        let wrld_mut = cell.world_mut();

        let mut go = wrld_mut.query::<&GameObject>();
        for g in go.iter(wrld){


            let mut code = g.code.clone();

            for t in Token::lexer(code.as_str()){
                for a in t.iter(){
                    if let Token::KeywordIf = a {
                        // get condition spanning spanning from
                        // whitespace seperator after if keyword
                        // to the first bracket to appear
                    }
                }
            }
        }
    }
}