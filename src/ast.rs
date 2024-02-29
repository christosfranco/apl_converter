//! enums and structures that store the syntax tree outputed by the parser.

use std::fmt;


// program        ::= EOF statement_list
// statement_list ::= (statement "⋄")* statement
// statement      ::= ( ID "←" | vector function | function )* vector
// function       ::= function mop | function dop f | f
// dop            ::= "∘" | "⍥"
// mop            ::= "⍨" | "¨"
// f              ::= "+" | "-" | "×" | "÷" | "⌈" | "⌊" |
//                  | "⊢" | "⊣" | "⍳" | "<" | "≤" | "=" |
//                  | "≥" | ">" | "≠" | "~" | "⊂" | LPARENS function RPARENS | dfn | fref
// dfn            ::= LBRACE statements RBRACE
// vector         ::= vector* ( scalar | ( LPARENS statement RPARENS ) )
// scalar         ::= INTEGER | FLOAT | COMPLEX | ID


// # chunk      ::= EOF statements
// # statements ::= (statement DIAMOND)* statement
// # statement  ::= ( ID GETS | vector function | function )* vector?
// # function   ::= function MOP | function DOP f | f
// # f          ::= FUN | LPAREN function RPAREN | dfn | fref
// # dfn        ::= LBRACE statements RBRACE
// # vector     ::= vector* ( scalar | ( LPAREN statement RPAREN ) )
// # scalar     ::= INTEGER | FLOAT | ID | ALPHA | OMEGA

// DYADIC_OPS = set('⍥@⍣⍤∘.⌺⍠') # FIXME: this should use Voc!!
// MONADIC_OPS = set('\\/⌿⍀¨⍨')

#[derive(Debug)]
pub enum StmtLst {
    Statement(Stmt),
    Lst(Stmt,Box<StmtLst>),
}

#[derive(Debug)]
pub enum Stmt {
    Assignment(Vector, Identifier),
    Function(Vector, Function),
    Vector(Vector),
    VectorFunction(Vector, Function, Vector)
}

#[derive(Debug)]
pub enum Function {
    Mop(Mop, Box<Function>),
    Dop(Dop, Box<Function>, F),
    BasicFunction(F),
}

#[derive(Debug)]
pub enum Dop {
    Composition, // "∘"
    CompositionWith, // "⍥"
}

#[derive(Debug)]
pub enum Mop {
    Reverse, // "⍨"
    Each, // "¨"
}

#[derive(Debug)]
pub enum F {
    Add,
    Subtract,
    Multiply,
    Divide,
    Ceil,
    Floor,
    TakeRight,
    TakeLeft,
    Index,
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    NotEqual,
    Negate,
    Enclose,
    Disclose,
    Indices,
}

#[derive(Debug)]
pub enum Vector {
    Multiple(Vec<Vector>, Scalar),
    Scalar(Scalar),
    Stmt(Box<Stmt>),
}

#[derive(Debug)]
pub enum Scalar {
    IntFloat(IntFloat),
    Complex(Complex ),
    Identifier(Identifier),
}
#[derive(Debug)]
pub enum Complex {
    Complex(IntFloat,IntFloat),
}

#[derive(Debug)]
pub enum IntFloat {
    Integer(i64),
    Float((i64,f64)),
}



#[derive(Debug)]
pub struct Identifier(
    pub String
);

/// Standard function in APL.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SFun {
    Assign,
    Conjugate,
    Plus,
    Negate,
    Minus,
    Direction,
    Times,
    Reciprocal,
    Divide,
    Exponential,
    Power,
    NaturalLogarithm,
    Logarithm,
    Comment,
}

impl fmt::Display for SFun {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match *self {
                SFun::Assign => "←",
                SFun::Conjugate => "+",
                SFun::Plus => "+",
                SFun::Negate => "-",
                SFun::Minus => "-",
                SFun::Direction => "×",
                SFun::Times => "×",
                SFun::Reciprocal => "÷",
                SFun::Divide => "÷",
                SFun::Exponential => "*",
                SFun::Power => "*",
                SFun::NaturalLogarithm => "⍟",
                SFun::Logarithm => "⍟",
                SFun::Comment => "⍝",
            }
        )
    }
}
// ← +-×÷*⍟ ⍝
// language syntax 
// <prefix> <symbol(s)>
// Assign, <prefix> [
// Conjugate, +
// Plus, +
// Negate, -
// Minus, -
// Direction, <prefix> -
// Times, <prefix> -
// Reciprocal, <prefix> =
// Divide, <prefix> =
// Exponential, <prefix> p
// Power, <prefix> p
// Natural Logarithm, <prefix> *
// Logarithm, <prefix> *
// Comment, <prefix> ,
// 
// 
// 
// 
//


