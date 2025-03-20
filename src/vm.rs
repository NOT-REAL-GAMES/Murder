use std::ptr::null;

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
        run_branch(code, Box::new(()));

        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct KeywordFunctionality {
    key: String,
    code: (),
}

pub fn run_branch(code:String,param:Box<dyn Any>) -> Box<dyn Any> {
    let mut t = Token::lexer(code.as_str());
    let mut idx = 0;
    while let Some(a) = t.next(){
        
        //println!("{a:?}");

        let func_kw = HashMap::from(
            [
                ("println".to_string(),(|param:Box<dyn Any>|{
                    let p: Box<String> = param.downcast().unwrap();
                    let mut s = *p;
                    s = s.trim_matches('\"').to_string();
                    println!("{s}");}))]);

        if let Ok(Token::KeywordIf) = a {
            // get condition spanning spanning from
            // whitespace seperator after if keyword
            // to the first bracket to appear

            // open vectors to push the data to
            let mut cond: Vec<Condition> = vec![];
            let mut cond_code: Vec<String>  = vec![];

            // index counter (obviously)
            let mut inner_idx = 0;
            
            //FIXME: this doesn't need a new lexer!!!
            let mut t2 = Token::lexer(code.as_str());
            while let Some(b) = t2.next(){
                if inner_idx <= idx{
                    inner_idx += 1;
                } else {
                    if let Ok(Token::BraceOpen) = b {
                        // FIXME: is supposed to push condition branches
                        //        but i haven't implemented it yet :(
                        cond.push(Condition{tokens:cond_code.concat()});
                        break;
                    } else {
                        if let Ok(Token::LogicOp(LGO::AND)) = b {
                            // TODO: implement condition branch
                        } else {
                            //FIXME: dumb dumb stupid
                            let fuck = t2.slice();
                            cond_code.push(fuck.to_string());
                        }
                        inner_idx += 1;
                    }
                }
            }
            //TODO: finish the actual code that's meant to compute the
            //      conditions (i.e. the other 99% of the interpreter)
            if (run_branch(cond[0].tokens.clone(),Box::new(()))).downcast().unwrap() == Box::new(true) {
                // iterate over every token and count how many levels of braces we're deep
                // if we're at the outmost layer and find a closing brace, copy code we have
                // run_branch(/* the code we've accumulated */)

                let mut exec_code_fragments: Vec<String> = vec![];

                let mut exec_idx = 0;

                let mut t = Token::lexer(code.as_str());
                while let Some(a) = t.next(){
                    if exec_idx > inner_idx{
                        if let Ok(Token::BraceClose) = a {
                            break;
                        } else {
                            exec_code_fragments.push(t.slice().to_string());
                        }
                    }
                    exec_idx+=1;
                }

                let exec_code = exec_code_fragments.concat();
                run_branch(exec_code, Box::new(()));
            }
        }
        else if let Ok(Token::Bool(true)) = a {
            // returns true if the code is just true
            // for use with if cases and while loops?
            // might add onto this if it creates problems
            // or if i think of some better method
            return Box::new(true);
        }
        else {
            //iterate through function keywords
            for (kw, code) in func_kw{
                if let Ok(Token::Text(ref kw)) = a {
                    let mut params = vec![];
                    let mut gathering = false;
                    while let Some(mut b) = t.next(){
                        if let Ok(Token::ParenOpen) = b{
                            gathering = true;
                        }
                        if gathering{
                            let mut raw = t.slice().to_string();
                            if let Ok(Token::String(ref mut raw)) = b {
                                raw.trim_matches('\"');
                                params.push(raw.clone());
                            }
                        }

                        if let Ok(Token::ParenClose) = b {
                            break;
                        }
                    }
                    let c = code(Box::new(params.join("")));
                }
            }
        }
        


        idx+=1;
    }
    return Box::new("fuck");

}