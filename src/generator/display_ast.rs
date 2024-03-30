use crate::generator::ast::*;
use std::fmt;

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