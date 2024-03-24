

use std::error::Error as StdError;


use clap::Result;
use nom::combinator::fail;

// pub mod ast;
// use apl_converter::generator::ast;
// use apl_converter::generator::ast;
use crate::generator::ast::*;
use crate::generator::ast::{StmtLst,Stmt,LeftStmt,Function,F,Vector,Scalar,Complex,Identifier,IntFloat,Mop,Dop};


pub fn generate_py(stmtlst : StmtLst) -> Result<String, Box<dyn StdError>> {
    let res = stmtlst.to_python();
    return Ok(res);
}

pub fn string_to_stmtlst(string :String) -> Result<StmtLst, Box<dyn StdError>> {
    
    return Ok(StmtLst::Statement(None, Stmt::LeftStmt(Vector::Scalar(None, Scalar::Identifier(Identifier("hi".to_string()))), None)))
}

impl StmtLst {
    fn to_python(&self) -> String {
        match self {
            StmtLst::Statement(_, stmt) => stmt.to_python(),
        }
    }
}

impl Stmt {
    fn to_python(&self) -> String {
        match self {
            Stmt::LeftStmt(vector, left_stmt_ref) => {
                let vector_str = vector.to_python();
                let left_stmt_vec_str: String = left_stmt_ref.iter().flatten()
                    .map(|stmt| stmt.to_python())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}{}", vector_str, left_stmt_vec_str)
            }
        }
    }
}

impl LeftStmt {
    fn to_python(&self) -> String {
        match self {
            LeftStmt::Assignment(scalar) => scalar.to_python(),
            LeftStmt::Function(function) => function.to_python(),
            LeftStmt::VectorFunction(vector, function) => {
                let vector_str = vector.to_python();
                let function_str = function.to_python();
                format!("{}{}", vector_str, function_str)
            }
        }
    }
}

impl Function {
    fn to_python(&self) -> String {
        match self {
            Function::Mop(mop, inner_function) => {
                let inner_function_str = inner_function.to_python();
                match mop {
                    Mop::Reverse => format!("reverse({})", inner_function_str),
                    Mop::Each => format!("each({})", inner_function_str),
                }
            }
            Function::Dop(dop, left_function, right_function) => {
                let left_function_str = left_function.to_python();
                let right_function_str = right_function.to_python();
                match dop {
                    Dop::Composition => format!("composition({}, {})", left_function_str, right_function_str),
                    Dop::CompositionWith => format!("composition_with({}, {})", left_function_str, right_function_str),
                }
            }
            Function::BasicFunction(f) => f.to_python(),
        }
    }
}

impl F {
    fn to_python(&self) -> String {
        match self {
            F::Add => "+".to_string(),
            F::Subtract => "-".to_string(),
            F::Multiply => "*".to_string(),
            F::Divide => "/".to_string(),
            // Handle other functions similarly
            _ => unimplemented!(),
        }
    }
}

impl Vector {
    fn to_python(&self) -> String {
        match self {
            Vector::Scalar(_, scalar) => scalar.to_python(),
            Vector::Stmt(_, stmt) => stmt.to_python(),
        }
    }
}

impl Scalar {
    fn to_python(&self) -> String {
        match self {
            Scalar::Complex(complex) => complex.to_python(),
            Scalar::IntFloat(int_float) => int_float.to_python(),
            Scalar::Identifier(identifier) => identifier.to_python(),
        }
    }
}

impl Complex {
    fn to_python(&self) -> String {
        match self {
            Complex::Complex(real, imaginary) => format!("({}+{}j)", real.to_python(), imaginary.to_python()),
        }
    }
}

impl IntFloat {
    fn to_python(&self) -> String {
        match self {
            IntFloat::Integer(i) => i.to_string(),
            IntFloat::Float((i, f)) => format!("{}.{}", i, f),
        }
    }
}

impl Identifier {
    fn to_python(&self) -> String {
        self.0.clone()
    }
}

#[test]
fn convert_test() {
    let ast = StmtLst::Statement(None, Stmt::LeftStmt(Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(7))), None)) ; 
    let res = generate_py(ast);
    let expected = "7";
    match res {
        Ok(string) => assert_eq!(string , expected ),
        Err(error) => panic!("{}",error),
    }
}
