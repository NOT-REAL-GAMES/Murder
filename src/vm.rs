use nom::Input;

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

#[derive(Logos, Debug, PartialEq)]
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
    #[token("&&", |_| LGO::AND)]
    #[token("||", |_| LGO::OR)]
    LogicOp(LGO),
    #[regex(r#"\"([a-zA-Z,.:;!\[\]\{\} ]*)\""#, |x| x.slice().to_owned(), priority = 1)]
    String(String),
    #[regex(r#"([a-zA-Z]*)"#, |x| x.slice().to_owned(), priority = 1)]
    Text(String),
    #[regex(r"[\s\t\n]*")]
    Whitespace
}

#[derive(Debug, PartialEq)]
enum LGO{
    AND,
    OR,
}

pub struct Condition{
    tokens: String,
}

pub struct ConditionBranch{

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
        run_branch(code);

        }
    }
}

pub fn run_branch(code:String) -> Box<dyn Any> {
    let mut t = Token::lexer(code.as_str());
    let mut idx = 0;
    while let Some(a) = t.next(){
        if let Ok(Token::KeywordIf) = a {
            // get condition spanning spanning from
            // whitespace seperator after if keyword
            // to the first bracket to appear

            
            let mut cond: Vec<Condition> = vec![];
            let mut v: Vec<String>  = vec![];
            let mut inner_idx = 0;
            let mut t2 = Token::lexer(code.as_str());
            while let Some(b) = t2.next(){
                if inner_idx <= idx{
                    inner_idx += 1;
                } else {
                    if let Ok(Token::BraceOpen) = b {
                        cond.push(Condition{tokens:v.concat()});
                        break;
                    } else {
                        if let Ok(Token::Whitespace) = b {

                        } else if let Ok(Token::LogicOp(LGO::AND)) = b {

                        } else {
                            //FIXME: dumb dumb stupid
                            let fuck = t2.slice();
                            v.push(fuck.to_string());
                        }
                        inner_idx += 1;
                    }
                }
            }
            //TODO: finish the actual code that's meant
            //      to compute the conditions (i.e. the
            //      other 99% of the interpreter)
            if (run_branch(cond[0].tokens.clone())).downcast().unwrap() == Box::new(true) {
                // iterate over every token and count how many levels of braces we're deep
                // if we're at the outmost layer and find a closing brace, copy code we have
                // run_branch(/* the code we've accumulated */)
            }
        }
        else if let Ok(Token::Bool(true)) = a {
            return Box::new(true);
        }
        idx+=1;
    }
    return Box::new("fuck");

}