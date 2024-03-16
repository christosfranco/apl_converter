extern crate nom;
use apl_converter::ast::*;
use nom::IResult;
use std::error::Error as StdError;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;


// use parser lib
pub mod parser_lib;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "APL Parser",
    about = "APL Parser is a Parser tool to convert APL code to an Abstract syntax tree."
)]

pub struct ProgArgs {
    /// Code file. Example: examples/hello_world.apl. If not set will expect `input_code`
    #[structopt(short, long)]
    file: Option<PathBuf>,

    /// Input code string. Use this if `file` is not set.
    #[structopt(short, long)]
    input: Option<String>,
}

fn read_file_contents(path: String) -> Result<String, Box<dyn StdError>> {
    // Read the file contents into a String
    let contents = fs::read_to_string(path)?;

    Ok(contents)
}

// Result<(&str, Vec<Result<(&str, apl_converter::ast::StmtLst), nom::Err<nom::error::Error<&str>>>>), _>
fn parse_lines2(input: Vec<&str>) -> Vec<IResult<&str, StmtLst>> {
    let mut vec_lines: Vec<IResult<&str, StmtLst>> = Vec::new();

    for line in input {
        vec_lines.push(parser_lib::parse_line(line))
    }
    vec_lines
}

fn main() -> Result<(), Box<dyn StdError>> {
    let args = ProgArgs::from_args();
    let code = args.input;
    let file = args.file;
    // TODO return Err instead of Ok if error happens with appropiate new error type
    match (code, file) {
        (Some(_), Some(_)) => eprintln!("Cannot give both code and file as input"),
        (None, None) => eprintln!("Either code or file expected as input"),
        (None, Some(file)) => {
            if let Some(file) = file.to_str() {
                match read_file_contents(file.to_string()) {
                    // VALID FILE INPUT
                    Ok(contents) => {
                        let rev_string = parser_lib::split_str_reverse_lines(&contents);
                        let mut input: Vec<&str> = Vec::new();
                        for line in rev_string.iter() {
                            input.push(line.as_str());
                        }
                        let res = parse_lines2(input);
                        // TODO return result here , instead of printing
                        println!("Result: {:?}", res);
                    }
                    Err(e) => {
                        eprintln!("Error reading file: {}", e);
                    }
                };
            } else {
                eprintln!("Code input was not valid format");
                // return Err((""))
            }
        }
        (Some(code), None) => {
            // VALID STRING INPUT
            let rev_string = parser_lib::split_str_reverse_lines(&code);
            let mut input: Vec<&str> = Vec::new();
            for line in rev_string.iter() {
                input.push(line.as_str());
            }
            let res = parse_lines2(input);
            // TODO return result here , instead of printing
            println!("Result: {:?}", res);
        }
    }
    Ok(())
}

#[test]
fn test_parse_matrix_nested2() {
    let string = "7 ((8 9) 10)";
    // reverse the input as references to adhere to the borrowchecker
    let rev_string = parser_lib::split_str_reverse_lines(string);

    let mut input: Vec<&str> = Vec::new();
    for line in rev_string.iter() {
        input.push(line.as_str());
    }
    let vector0: Vector = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(7)));
    let vecstmt8: Vector = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(8)));
    let stmt89: Vector = Vector::Stmt(
        None,
        Box::new(Stmt::LeftStmt(
            Vector::Scalar(
                Some(Box::new(vecstmt8)),
                Scalar::IntFloat(IntFloat::Integer(9)),
            ),
            None,
        )),
    );

    let vec10: Vector = Vector::Scalar(
        Some(Box::new(stmt89)),
        Scalar::IntFloat(IntFloat::Integer(10)),
    );
    let stmt: Stmt = Stmt::LeftStmt(vec10, None);
    let vector: Vector = Vector::Stmt(Some(Box::new(vector0)), Box::new(stmt));
    let expected: Result<(&str, StmtLst), nom::Err<nom::error::Error<&str>>> =
        Ok(("", StmtLst::Statement(None, Stmt::LeftStmt(vector, None))));
    let mut expected_vec = Vec::new();
    expected_vec.push(expected);
    let actual_vec = parse_lines2(input);
    // println!("Actual: {:?}", actual_vec);
    // println!("Expected: {:?}", expected_vec);
    for (actual, expected) in actual_vec.iter().zip(expected_vec.iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_parse_matrix_nested() {
    let string = "7 ((9) 10)";

    // reverse the input as references to adhere to the borrowchecker
    let rev_string = parser_lib::split_str_reverse_lines(string);

    let mut input: Vec<&str> = Vec::new();
    for line in rev_string.iter() {
        input.push(line.as_str());
    }
    let vector0: Vector = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(7)));
    let stmt89: Vector = Vector::Stmt(
        None,
        Box::new(Stmt::LeftStmt(
            Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(9))),
            None,
        )),
    );
    let vec10: Vector = Vector::Scalar(
        Some(Box::new(stmt89)),
        Scalar::IntFloat(IntFloat::Integer(10)),
    );
    let stmt: Stmt = Stmt::LeftStmt(vec10, None);
    let vector: Vector = Vector::Stmt(Some(Box::new(vector0)), Box::new(stmt));
    let expected: Result<(&str, StmtLst), nom::Err<nom::error::Error<&str>>> =
        Ok(("", StmtLst::Statement(None, Stmt::LeftStmt(vector, None))));
    let mut expected_vec = Vec::new();
    expected_vec.push(expected);
    let actual_vec = parse_lines2(input);
    // println!("Actual: {:?}", actual_vec);
    // println!("Expected: {:?}", expected_vec);

    for (actual, expected) in actual_vec.iter().zip(expected_vec.iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_parse_matrix() {
    let string = "7 (10)";
    // reverse the input as references to adhere to the borrowchecker
    let rev_string = parser_lib::split_str_reverse_lines(string);

    let mut input: Vec<&str> = Vec::new();
    for line in rev_string.iter() {
        input.push(line.as_str());
    }
    let vector0: Vector = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(7)));
    let vec10: Vector = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(10)));
    let stmt: Stmt = Stmt::LeftStmt(vec10, None);
    let vector: Vector = Vector::Stmt(Some(Box::new(vector0)), Box::new(stmt));
    let expected: Result<(&str, StmtLst), nom::Err<nom::error::Error<&str>>> =
        Ok(("", StmtLst::Statement(None, Stmt::LeftStmt(vector, None))));
    let mut expected_vec = Vec::new();
    expected_vec.push(expected);
    let actual_vec = parse_lines2(input);
    // println!("Actual: {:?}", actual_vec);
    // println!("Expected: {:?}", expected_vec);
    for (actual, expected) in actual_vec.iter().zip(expected_vec.iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_parse_lines_multiples() {
    // let string = "  id3 ← 3 ⋄  id2 ←id1 ←  1";
    let string = "  id3 ← 3 ⋄  id2 ←id1 ←  1
     id2 ←id1 ←  1 ";
    // reverse the input as references to adhere to the borrowchecker
    let rev_string = parser_lib::split_str_reverse_lines(string);

    let mut input: Vec<&str> = Vec::new();
    for line in rev_string.iter() {
        input.push(line.as_str());
    }

    // Line 0
    // RHS of Statement
    let rhs_vec = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(1)));
    let rhs_assignment: Option<Vec<LeftStmt>> = Some(vec![
        LeftStmt::Assignment(Scalar::Identifier(Identifier("id1".to_string()))),
        LeftStmt::Assignment(Scalar::Identifier(Identifier("id2".to_string()))),
    ]);

    // LHS of Statement
    let lhs_vec = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(3)));
    let lhs_assignment: Option<Vec<LeftStmt>> = Some(vec![LeftStmt::Assignment(
        Scalar::Identifier(Identifier("id3".to_string())),
    )]);
    let lhs: Option<Vec<Stmt>> = Some(vec![Stmt::LeftStmt(lhs_vec, lhs_assignment)]);
    let expected: Result<(&str, StmtLst), nom::Err<nom::error::Error<&str>>> = Ok((
        "",
        StmtLst::Statement(lhs, Stmt::LeftStmt(rhs_vec, rhs_assignment)),
    ));

    // Line 1
    let snd = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(1)));
    let assignment: Option<Vec<LeftStmt>> = Some(vec![
        LeftStmt::Assignment(Scalar::Identifier(Identifier("id1".to_string()))),
        LeftStmt::Assignment(Scalar::Identifier(Identifier("id2".to_string()))),
    ]);
    let expected1: Result<(&str, StmtLst), nom::Err<nom::error::Error<&str>>> = Ok((
        "",
        StmtLst::Statement(None, Stmt::LeftStmt(snd, assignment)),
    ));

    // Complete vector with all lines
    let mut expected_vec = Vec::new();
    expected_vec.push(expected);
    expected_vec.push(expected1);

    let actual_vec = parse_lines2(input);
    // println!("Actual: {:?}", actual_vec);
    // println!("Expected: {:?}", expected_vec);

    assert_eq!(actual_vec.len(), expected_vec.len());

    for (actual, expected) in actual_vec.iter().zip(expected_vec.iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_parse_lines() {
    let string = "  id3 ← 3 ⋄  id2 ←id1 ←  1";
    // let string = "  id3 ← 3 ⋄  id2 ←id1 ←  1 \n   id2 ←id1 ←  1 ";

    // reverse the input as references to adhere to the borrowchecker
    let rev_string = parser_lib::split_str_reverse_lines(string);

    let mut input: Vec<&str> = Vec::new();
    for line in rev_string.iter() {
        input.push(line.as_str());
    }

    // RHS of Statement
    let rhs_vec = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(1)));
    let rhs_assignment: Option<Vec<LeftStmt>> = Some(vec![
        LeftStmt::Assignment(Scalar::Identifier(Identifier("id1".to_string()))),
        LeftStmt::Assignment(Scalar::Identifier(Identifier("id2".to_string()))),
    ]);

    // LHS of Statement
    let lhs_vec = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(3)));
    let lhs_assignment: Option<Vec<LeftStmt>> = Some(vec![LeftStmt::Assignment(
        Scalar::Identifier(Identifier("id3".to_string())),
    )]);
    let lhs: Option<Vec<Stmt>> = Some(vec![Stmt::LeftStmt(lhs_vec, lhs_assignment)]);
    let expected: Result<(&str, StmtLst), nom::Err<nom::error::Error<&str>>> = Ok((
        "",
        StmtLst::Statement(lhs, Stmt::LeftStmt(rhs_vec, rhs_assignment)),
    ));

    let mut expected_vec = Vec::new();
    expected_vec.push(expected);

    let actual_vec = parse_lines2(input);
    // println!("Actual: {:?}", actual_vec);
    // println!("Expected: {:?}", expected_vec);

    assert_eq!(actual_vec.len(), expected_vec.len());

    for (actual, expected) in actual_vec.iter().zip(expected_vec.iter()) {
        assert_eq!(actual, expected);
    }
}
