use crate::ast::*;

// Implement PartialEq for StmtLst enum
impl PartialEq for StmtLst {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StmtLst::Statement(stmtlst1, stmt1), StmtLst::Statement(stmtlst2, stmt2)) => {
                stmt1 == stmt2 && stmtlst1 == stmtlst2
            }
            // (StmtLst::Lst(stmt1, box_stmtlst1), StmtLst::Lst(stmt2, box_stmtlst2)) => {
            //     stmt1 == stmt2 && *box_stmtlst1 == *box_stmtlst2
            // }
            // _ => false,
        }
    }
}

// Implement PartialEq for Stmt enum
impl PartialEq for Stmt {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Stmt::LeftStmt(vector1, left_stmts1), Stmt::LeftStmt(vector2, left_stmts2)) => {
                vector1 == vector2 && left_stmts1 == left_stmts2
            } // _ => false,
        }
    }
}

// TODO can the order of the leftstatements be permutated (different order) and still be equal??
impl PartialEq for LeftStmt {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LeftStmt::Assignment(id1), LeftStmt::Assignment(id2)) => id1 == id2,
            (LeftStmt::Function(func1), LeftStmt::Function(func2)) => func1 == func2,
            (LeftStmt::VectorFunction(vec1, func1), LeftStmt::VectorFunction(vec2, func2)) => {
                vec1 == vec2 && func1 == func2
            }
            _ => false,
        }
    }
}

// Implement PartialEq for Function enum
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Function::Mop(mop1, box_func1), Function::Mop(mop2, box_func2)) => {
                mop1 == mop2 && *box_func1 == *box_func2
            }
            (Function::Dop(dop1, box_func1, f1), Function::Dop(dop2, box_func2, f2)) => {
                dop1 == dop2 && *box_func1 == *box_func2 && f1 == f2
            }
            (Function::BasicFunction(f1), Function::BasicFunction(f2)) => f1 == f2,
            _ => false,
        }
    }
}

// Implement PartialEq for Dop enum
impl PartialEq for Dop {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Dop::Composition, Dop::Composition) => true,
            (Dop::CompositionWith, Dop::CompositionWith) => true,
            _ => false,
        }
    }
}

// Implement PartialEq for Mop enum
impl PartialEq for Mop {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Mop::Reverse, Mop::Reverse) => true,
            (Mop::Each, Mop::Each) => true,
            _ => false,
        }
    }
}

// Implement PartialEq for F enum
impl PartialEq for F {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (F::Add, F::Add) => true,
            (F::Subtract, F::Subtract) => true,
            (F::Multiply, F::Multiply) => true,
            // Implement PartialEq for other variants...
            _ => false,
        }
    }
}

// Implement PartialEq for Vector enum
impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Vector::Scalar(vec1, scalar1), Vector::Scalar(vec2, scalar2)) => {
                scalar1 == scalar2 && vec1 == vec2
            }
            (Vector::Stmt(vec1, box_stmt1), Vector::Stmt(vec2, box_stmt2)) => {
                box_stmt1 == box_stmt2 && vec1 == vec2
            }
            _ => false,
        }
    }
}

// Implement PartialEq for Scalar enum
impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Scalar::IntFloat(int_float1), Scalar::IntFloat(int_float2)) => {
                int_float1 == int_float2
            }
            (Scalar::Complex(complex1), Scalar::Complex(complex2)) => complex1 == complex2,
            (Scalar::Identifier(id1), Scalar::Identifier(id2)) => id1 == id2,
            _ => false,
        }
    }
}

// Implement PartialEq for Complex enum
impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Complex::Complex(int_float1a, int_float1b),
                Complex::Complex(int_float2a, int_float2b),
            ) => int_float1a == int_float2a && int_float1b == int_float2b,
        }
    }
}

// Implement PartialEq for IntFloat enum
impl PartialEq for IntFloat {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (IntFloat::Integer(i1), IntFloat::Integer(i2)) => i1 == i2,
            (IntFloat::Float((i1, f1)), IntFloat::Float((i2, f2))) => i1 == i2 && f1 == f2,
            _ => false,
        }
    }
}

// Implement PartialEq for Identifier struct
impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
