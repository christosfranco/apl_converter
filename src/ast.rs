//! enums and structures that store the syntax tree outputed by the parser.

use std::fmt;


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


