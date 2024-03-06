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


/// recursive parsing structure instead of 0 or many
// program        ::= EOF statement_list
// statement_list ::= (statement_list "⋄" | None) statement
// statement      ::= (left_statement | None)     vector
// left_statement ::= Vec<( ID "←" | vector function | function )>
// function       ::= function mop | function dop f | f
// dop            ::= "∘" | "⍥"
// mop            ::= "⍨" | "¨"
// f              ::= "+" | "-" | "×" | "÷" | "⌈" | "⌊" |
//                  | "⊢" | "⊣" | "⍳" | "<" | "≤" | "=" |
//                  | "≥" | ">" | "≠" | "~" | "⊂" | LPARENS function RPARENS | dfn | fref
// dfn            ::= LBRACE statements RBRACE
// vector         ::= (vector | None) ( scalar | ( LPARENS statement RPARENS ) )
// scalar         ::= INTEGER | FLOAT | COMPLEX | ID


// vector         ::= vector vector scalar
// vector         ::= vector scalar scalar


// parse stmt
// parse "⋄" if true
    // then parse stmtlst

// statement_list ::= (statement "⋄") (statement "⋄") statement
#[derive(Debug)]
pub enum StmtLst {
    Statement(Option<Box<StmtLst>>,Stmt),
}

#[derive(Debug)]
pub enum Stmt {
    LeftStmt(Vector, Option<Vec<LeftStmt>>),
}

#[derive(Debug)]
pub enum LeftStmt {
    // TODO refactor to match type Identifier instead of Scalar
    Assignment(Scalar),
    Function(Function),
    VectorFunction(Vector,Function)
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

// 0 or many vectors
// call parse_vector recursively
#[derive(Debug)]
pub enum Vector {
    // Multiple(Vec<Vector>),
    Scalar(Option<Box<Vector>>, Scalar),
    Stmt(Option<Box<Vector>>,Box<Stmt>),
}

#[derive(Debug)]
pub enum Scalar {
    Complex(Complex ),
    IntFloat(IntFloat),
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





// OLD AST, KEEP until Display function is refactored

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


