#![allow(unused_imports)]

use std::env;
use std::fs;
use std::io::prelude::*;
use regex::Regex;

// Token Codes (punctuation corresponding to utf8 decimal codes)
const POINT: i32 = 1;
const ID: i32 = 2; 
const NUM: i32 = 3; 
const SEMICOLON: i32 = 4;
const COMMA: i32 = 5;
const PERIOD: i32 = 6;
const L_PAREN: i32 = 7;
const R_PAREN: i32 = 8;
const ASSIGN: i32 = 9;

//Stores each token
static mut TOKENS: Vec<i32> = Vec::new();
//Stores each number 
static mut NUMS: Vec<i32> = Vec::new();

// Regular Expressions for Syntax Analysis
// let sentence = Regex::new(r"[[:alpha:]]+=point(\d,\d);");


//HELPER FUNCTIONS FOR PARSING INPUT TEXT
fn is_alpha(c: char) -> bool {
    let re = Regex::new(r"[[:alpha:]]").unwrap();
    return re.is_match(&c.to_string());
}

fn is_digit(c: char) -> bool {
    let re = Regex::new(r"\d").unwrap();
    return re.is_match(&c.to_string());
}

fn is_space(c: char) -> bool{
    let re = Regex::new(r"\s").unwrap();
    return re.is_match(&c.to_string());
}

unsafe fn id_or_num(l: &String){
    let num = Regex::new(r"\d+").unwrap();
    let id = Regex::new(r"[[:alpha:]]+").unwrap();
    
    if id.is_match(l){
        if l == "point"{
            TOKENS.push(POINT);
        }else{
            TOKENS.push(ID);
        }
    }else if num.is_match(l){
        TOKENS.push(NUM);
        NUMS.push(l.parse::<i32>().unwrap());
    }
}

// LEXICAL ANALYZER
fn lex(contents: &String, lexeme: &mut String) {
    let contents_to_vec: Vec<char> = contents.chars().collect();
    for i in 0..contents_to_vec.len(){
        if is_alpha(contents_to_vec[i]) || is_digit(contents_to_vec[i]) {
            lexeme.push(contents_to_vec[i]);
        }else if is_space(contents_to_vec[i]) {
            if lexeme.len() > 0{
                unsafe{id_or_num(lexeme);}
                *lexeme = String::from("");
            }
            continue;
        }else{
            if lexeme.len() > 0{
                unsafe{id_or_num(lexeme);}
                *lexeme = String::from("");
            }
            unsafe {lookup(contents_to_vec[i], i);}
        }
    }
}

unsafe fn lookup (c: char, index: usize){
    match c {
        '(' => TOKENS.push(L_PAREN),
        ')' => TOKENS.push(R_PAREN),
        '=' => TOKENS.push(ASSIGN),
        '.' => TOKENS.push(PERIOD),
        ',' => TOKENS.push(COMMA),
        ';' => TOKENS.push(SEMICOLON),
        _ => panic!("LEXICAL ERROR: Illegal Character {} at position {}", c, index)
    }
} 

// SYNTAX ANALYZER
unsafe fn syntax_analysis(){

    for i in 0..TOKENS.len()-1{
        if TOKENS[i] == ID && TOKENS[i+1] != ASSIGN{
            panic!("SYNTAX ERROR: Invalid token {} at position {}",TOKENS[i], i);
        }
        if TOKENS[i] == ASSIGN && TOKENS[i+1] != POINT{
            panic!("SYNTAX ERROR: Invalid token {} at position {}",TOKENS[i], i);
        }
        if TOKENS[i] == POINT && TOKENS[i+1] != L_PAREN{
            panic!("SYNTAX ERROR: Invalid token {} at position {}",TOKENS[i], i);
        }
        if TOKENS[i] == L_PAREN && TOKENS[i+1] != NUM{
            panic!("SYNTAX ERROR: Invalid token {} at position {}",TOKENS[i], i);
        }
        if TOKENS[i] == NUM{
            if TOKENS[i+1] == COMMA || TOKENS[i+1] == R_PAREN{
                continue;
            }
            panic!("SYNTAX ERROR: Invalid token {} at position {}",TOKENS[i], i);
        }
        if TOKENS[i] == COMMA && TOKENS[i+1] != NUM{
            panic!("SYNTAX ERROR: Invalid token {} at position {}",TOKENS[i], i);
        }
        if TOKENS[i] == R_PAREN && TOKENS[i+1] != SEMICOLON{
            if TOKENS[i+1] != PERIOD{
                panic!("SYNTAX ERROR: Invalid token {} at position {}",TOKENS[i], i);
            }
        }
        if TOKENS[i] == SEMICOLON && TOKENS[i+1] != ID{
            panic!("SYNTAX ERROR: Invalid token {} at position {}",TOKENS[i], i);
        }
        if TOKENS[i+1] == PERIOD && i+1 < TOKENS.len()-1{
            panic!("SYNTAX ERROR: Invalid token {} at position {}",TOKENS[i], i);
        }
    }
}

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();

    //Stores each lexeme
    let mut lexeme = String::from("");

    let file_contents = fs::read_to_string(&args[1])
        .expect("File read failed!");

    if args.len() == 3{
        if args[2] == "-p"{
            println!("/* processing input file {}", &args[1]);
        } else if args[2] == "-s"{
            println!("; Processing input file {}", &args[1]);
        }else{
            panic!("Unknown flag {}", &args[2]);
        }
    }

    //Lexical Analysis
    lex(&file_contents, &mut lexeme);
    
    //Syntax Analysis
    unsafe {
        syntax_analysis();
    }

    unsafe{
        if NUMS.len() == 6{
            if args.len() == 3{
                if args[2] == "-p"{
                    prolog(NUMS[0], NUMS[1], NUMS[2], NUMS[3],NUMS[4], NUMS[5]);
                } else if args[2] == "-s"{
                    scheme(NUMS[0], NUMS[1], NUMS[2], NUMS[3],NUMS[4], NUMS[5]);
                }else{
                    panic!("Unknown flag {}", &args[2]);
                }
            }
        }else{
            println!("ERROR: Incorrect number of points");
        }
    }
    
    Ok(())
   
}

unsafe fn scheme(a0: i32, a1: i32, b0: i32, b1: i32, c0: i32, c1: i32){
    //End message
    println!("; Lexical and Syntax analysis passed");
    //Scheme function output
    println!("(calculate-triangle (make-point {} {}) (make-point {} {}) (make-point {} {}))", a0, a1, b0, b1, c0, c1);
}

unsafe fn prolog(a0: i32, a1: i32, b0: i32, b1: i32, c0: i32, c1: i32) {
    println!("Lexical and Syntax analysis passed */");

    println!("query(line(point2d({},{}), point2d({},{}), point2d({}, {})))", a0, a1, b0, b1, c0, c1);
    println!("query(triangle(point2d({},{}), point2d({},{}), point2d({}, {})))", a0, a1, b0, b1, c0, c1);
    println!("query(vertical(point2d({},{}), point2d({},{}), point2d({}, {})))", a0, a1, b0, b1, c0, c1);
    println!("query(horizontal(point2d({},{}), point2d({},{}), point2d({}, {})))", a0, a1, b0, b1, c0, c1);
    println!("query(equilateral(point2d({},{}), point2d({},{}), point2d({}, {})))", a0, a1, b0, b1, c0, c1);
    println!("query(isosceles(point2d({},{}), point2d({},{}), point2d({}, {})))", a0, a1, b0, b1, c0, c1);
    println!("query(right(point2d({},{}), point2d({},{}), point2d({}, {})))", a0, a1, b0, b1, c0, c1);
    println!("query(scalene(point2d({},{}), point2d({},{}), point2d({}, {})))", a0, a1, b0, b1, c0, c1);
    println!("query(acute(point2d({},{}), point2d({},{}), point2d({}, {})))", a0, a1, b0, b1, c0, c1);
    println!("query(obtuse(point2d({},{}), point2d({},{}), point2d({}, {})))", a0, a1, b0, b1, c0, c1);
    println!("writeln(T) :- write(T), nl.");
    println!("main:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),");
    println!("halt.");
}