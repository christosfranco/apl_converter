extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till, take_until, take_while, take_while_m_n},
    character::{
        complete::{
            alpha1, alphanumeric0, anychar, char, digit1, multispace0, newline, space0, space1,
        },
        is_space,
    },
    combinator::{map, map_res, opt, peek, recognize},
    error::{convert_error, Error, VerboseError},
    multi::{many0, many1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::ast;
use crate::ast::*;

pub fn split_str_reverse_lines(s: &str) -> Vec<String> {
    // Split the string into lines
    let vec_lines: Vec<String> = s.lines().map(|line| reverse(line).to_string()).collect();
    // let mut vec_lines = Vec::new();
    // vec_lines.push("something");
    // Print the reversed lines
    return vec_lines;
}

pub fn parse_line(input: &str) -> IResult<&str, StmtLst> {
    parse_statement_list(input)
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn reverse_line(input: &str) -> String {
    reverse(input)
}

fn parse_comment_content(input: &str) -> IResult<&str, &str> {
    preceded(tag("⍝"), take_until("\n"))(input)
}

fn parse_comment(input: &str) -> IResult<&str, &str> {
    parse_comment_content(input)
}

//// SCALAR PARSERS
fn parse_str_to_int(input: &str) -> IResult<&str, i64> {
    // let res = alt((terminated(digit1,tag("¯")),digit1))(input);
    match parse_negative(input) {
        Ok((remainder, (output, boolean))) => {
            let rev: String = reverse(output);
            let res = rev.parse::<i64>();
            match res {
                Ok(int) => match boolean {
                    true => Ok((remainder, -int)),
                    false => Ok((remainder, int)),
                },
                Err(_error) => Err(nom::Err::Failure(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Digit,
                ))),
            }
        }
        Err(error) => Err(error),
    }
}

fn parse_str_to_float(input: &str) -> IResult<&str, f64> {
    let zero: String = "0.".to_string();
    let res = alt((terminated(digit1, tag("¯")), digit1))(input);
    match res {
        Ok((remainder, output)) => {
            let rev: String = reverse(output);
            let combined: &str = &(zero + &rev);
            let res = combined.parse::<f64>();
            match res {
                Ok(float) => Ok((remainder, float)),
                Err(_error) => Err(nom::Err::Failure(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Float,
                ))),
            }
        }
        Err(error) => Err(error),
    }
}

/// Format Imaginary i ["j","J"] Real r , ijr
fn parse_complex(input: &str) -> IResult<&str, ast::Scalar> {
    match separated_pair(parse_intfloat, tag_no_case("j"), parse_intfloat)(input) {
        Ok((remainder, (first, second))) => {
            // real part is first, i part is second
            Ok((remainder, Scalar::Complex(Complex::Complex(second, first))))
        }
        Err(error) => Err(error),
    }
}

fn parse_float(input: &str) -> IResult<&str, ast::IntFloat> {
    match separated_pair(parse_str_to_float, tag("."), parse_str_to_int)(input) {
        Ok((remainder, (first, second))) => Ok((remainder, (IntFloat::Float((second, first))))),
        Err(error) => Err(error),
    }
}

fn parse_int(input: &str) -> IResult<&str, ast::IntFloat> {
    match parse_str_to_int(input) {
        Ok((remainder, second)) => Ok((remainder, (IntFloat::Integer(second)))),
        Err(error) => Err(error),
    }
}

/// bool = true => negative
fn parse_negative(input: &str) -> IResult<&str, (&str, bool)> {
    let res: IResult<&str, &str> = terminated(digit1, tag("¯"))(input);
    match res {
        Ok((remainder, output)) => return Ok((remainder, (output, true))),
        // todo combine errors
        Err(_error) => {
            let res: IResult<&str, &str> = digit1(input);
            match res {
                Ok((remainder, output)) => return Ok((remainder, (output, false))),
                Err(error2) => Err(error2),
            }
        }
    }
}

fn parse_intfloat(input: &str) -> IResult<&str, ast::IntFloat> {
    let res = alt((parse_float, parse_int))(input);
    match res {
        Ok((remainder, output)) => Ok((remainder, (output))),
        Err(error) => Err(error),
    }
}

fn parse_id(input: &str) -> IResult<&str, ast::Scalar> {
    let res: IResult<&str, &str> = recognize(alphanumeric0)(input);
    match res {
        Ok((remainder, output)) => match output.chars().last().unwrap_or(' ').is_alphabetic() {
            true => Ok((
                remainder,
                Scalar::Identifier(Identifier(reverse(output).to_string())),
            )),
            false => Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Alpha,
            ))),
        },
        Err(error) => Err(error),
    }
}

fn parse_scalar(input: &str) -> IResult<&str, ast::Scalar> {
    // let (_,res_string,_) = tuple( (char('\''), many1(anychar) ,char('\'') ));
    let alt_parser = alt((space0, alphanumeric0));
    match tuple((
        char::<&str, Error<&str>>('\''),
        many1(alt_parser),
        char('\''),
    ))(input)
    {
        Ok((remainder, (_c1, output_statement, _c2))) => {
            return Ok((
                remainder,
                Scalar::Identifier(Identifier(
                    output_statement.iter().cloned().collect::<String>(),
                )),
            ))
        }
        Err(_error) => {
            let res = alt((parse_complex, parse_id))(input);
            match res {
                Ok((remainder, output)) => Ok((remainder, (output))),
                Err(_error) => {
                    let res = parse_intfloat(input);
                    match res {
                        Ok((remainder, output)) => Ok((remainder, Scalar::IntFloat(output))),
                        Err(error) => Err(error),
                    }
                }
            }
        }
    }
}

// end SCALAR PARSERS

// TODO parse space1 after each scalar.
fn parse_vector(input: &str) -> IResult<&str, ast::Vector> {
    // println!("Parsing vector") ;
    let (input, _) = space0(input)?;
    match (parse_scalar)(input) {
        Ok((remainder, output_scalar)) => {
            match parse_vector(remainder) {
                Ok((remainder_vector, output_vector)) => {
                    return Ok((
                        remainder_vector,
                        Vector::Scalar(Some(Box::new(output_vector)), output_scalar),
                    ));
                }
                Err(_error) => return Ok((remainder, Vector::Scalar(None, output_scalar))),
            };
        }
        // TODO accumulate error if both parse_scalar and parse statement fails
        Err(_error_scalar) => match (tuple(((char(')')), parse_statement, char('('))))(input) {
            Ok((remainder, (_c1, output_statement, _c2))) => {
                match parse_vector(remainder) {
                    Ok((remainder, output_vector)) => {
                        return Ok((
                            remainder,
                            Vector::Stmt(Some(Box::new(output_vector)), Box::new(output_statement)),
                        ));
                    }
                    Err(_error) => {
                        return Ok((remainder, Vector::Stmt(None, Box::new(output_statement))))
                    }
                };
            }
            Err(error_statement) => return Err(error_statement),
        },
    };
}

fn parse_assignment(input: &str) -> IResult<&str, LeftStmt> {
    let parse_id_with_space = preceded(space0, parse_id);
    let (input, id1) = preceded(tag("←"), parse_id_with_space)(input)?;
    Ok((input, LeftStmt::Assignment(id1)))
}

fn parse_statement(input: &str) -> IResult<&str, Stmt> {
    // Define parsers with optional whitespace
    let parse_vector_with_space = preceded(space0, parse_vector);
    // let parse_id_with_space = preceded(space0, parse_id);

    let many0_id_parser = many1(preceded(space0, parse_assignment));
    let res = terminated(pair(parse_vector_with_space, opt(many0_id_parser)), space0)(input);
    // Todo make alternative for function and vectorfunction

    // Use `separated_pair` with `terminated` to parse statement with whitespace checks
    // let res= terminated(separated_pair(parse_vector_with_space, preceded(space0,char('←')), parse_id_with_space), space0)(input);
    match res {
        Ok((remainder, (vector, option_vec_leftstmt))) => {
            // match option_vec_leftstmt {
            //   Some(())
            // }
            Ok((remainder, Stmt::LeftStmt(vector, option_vec_leftstmt)))
        }
        Err(error) => Err(error),
    }
}

fn parse_statement_list(input: &str) -> IResult<&str, StmtLst> {
    match parse_statement(input) {
        Ok((remainder, output)) => match many1(preceded(char('⋄'), parse_statement))(remainder) {
            Ok((remainder, output_vec_stmt)) => {
                Ok((remainder, StmtLst::Statement(Some(output_vec_stmt), output)))
            }
            Err(_error) => Ok((remainder, StmtLst::Statement(None, output))),
        },
        // it is only an error if it fails to pass the first statement
        Err(error) => Err(error),
    }
}

#[test]
fn test_parse_stmt_lst() {
    let string = "  id3 ← 3 ⋄  id2 ←id1 ←  1 ";
    let input = &reverse_line(string);

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

    let actual = parse_statement_list(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_stmt_lst_single() {
    let string = "   id2 ←id1 ←  1 ";
    let input = &reverse_line(string);

    let snd = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(1)));
    let assignment: Option<Vec<LeftStmt>> = Some(vec![
        LeftStmt::Assignment(Scalar::Identifier(Identifier("id1".to_string()))),
        LeftStmt::Assignment(Scalar::Identifier(Identifier("id2".to_string()))),
    ]);

    let expected: Result<(&str, StmtLst), nom::Err<nom::error::Error<&str>>> = Ok((
        "",
        StmtLst::Statement(None, Stmt::LeftStmt(snd, assignment)),
    ));

    let actual = parse_statement_list(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_multi_assign_space() {
    let string = "  id2 ←id1 ←  1 ";
    let input = &reverse_line(string);

    let snd = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(1)));
    let assignment: Option<Vec<LeftStmt>> = Some(vec![
        LeftStmt::Assignment(Scalar::Identifier(Identifier("id1".to_string()))),
        LeftStmt::Assignment(Scalar::Identifier(Identifier("id2".to_string()))),
    ]);

    let expected: Result<(&str, Stmt), nom::Err<nom::error::Error<&str>>> =
        Ok(("", Stmt::LeftStmt(snd, assignment)));

    let actual = parse_statement(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_assign_space() {
    let string = "   id1 ←  1 ";
    let input = &reverse_line(string);

    let snd = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(1)));
    let assignment: Option<Vec<LeftStmt>> = Some(vec![LeftStmt::Assignment(Scalar::Identifier(
        Identifier("id1".to_string()),
    ))]);

    let expected: Result<(&str, Stmt), nom::Err<nom::error::Error<&str>>> =
        Ok(("", Stmt::LeftStmt(snd, assignment)));

    let actual = parse_statement(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_assign_multiple_space() {
    let string = " id1  ← 1 1J2.03";
    let input = &reverse_line(string);

    let snd = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(1)));
    let fst = Vector::Scalar(
        Some(Box::new(snd)),
        Scalar::Complex(Complex::Complex(
            IntFloat::Integer(1),
            IntFloat::Float((2, 0.03)),
        )),
    );
    let assignment: Option<Vec<LeftStmt>> = Some(vec![LeftStmt::Assignment(Scalar::Identifier(
        Identifier("id1".to_string()),
    ))]);

    let expected: Result<(&str, Stmt), nom::Err<nom::error::Error<&str>>> =
        Ok(("", Stmt::LeftStmt(fst, assignment)));

    let actual = parse_statement(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_vector_single() {
    let string = "1";
    let input = &reverse_line(string);
    let fst = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(1)));
    let expected: Result<(&str, Vector), nom::Err<nom::error::Error<&str>>> = Ok(("", fst));
    let actual = parse_vector(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_vector_multiple() {
    let string = "1    2   ";
    let input = &reverse_line(string);

    let snd = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(1)));
    let fst = Vector::Scalar(Some(Box::new(snd)), Scalar::IntFloat(IntFloat::Integer(2)));
    let expected: Result<(&str, Vector), nom::Err<nom::error::Error<&str>>> = Ok(("", fst));
    let actual = parse_vector(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

// TESTS
#[test]
fn test_parse_float() {
    // assuming lines are reversed?
    let string = "312.23311";
    let input = &reverse_line(string);
    let output: ast::IntFloat = IntFloat::Float((312, 0.23311));
    let expected: Result<(&str, ast::IntFloat), nom::Err<nom::error::Error<&str>>> =
        Ok(("", output));
    let actual = parse_intfloat(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_int() {
    // assuming lines are reversed?
    let string = "312";
    let input = &reverse_line(string);
    let output: ast::IntFloat = IntFloat::Integer(312);
    let expected: Result<(&str, ast::IntFloat), nom::Err<nom::error::Error<&str>>> =
        Ok(("", output));
    let actual = parse_intfloat(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_complex_int_int() {
    let string = "31J23";
    let input = &reverse_line(string);
    let output: ast::Scalar = Scalar::Complex(Complex::Complex(
        IntFloat::Integer(31),
        IntFloat::Integer(23),
    ));
    let expected: Result<(&str, ast::Scalar), nom::Err<nom::error::Error<&str>>> = Ok(("", output));
    let actual = parse_complex(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_complex_float_int() {
    let string = "31.23J223";
    let input = &reverse_line(string);
    let output: ast::Scalar = Scalar::Complex(Complex::Complex(
        IntFloat::Float((31, 0.23)),
        IntFloat::Integer(223),
    ));
    let expected: Result<(&str, ast::Scalar), nom::Err<nom::error::Error<&str>>> = Ok(("", output));
    let actual = parse_complex(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_complex_int_float() {
    let string = "301J21.89";
    let input = &reverse_line(string);
    let output: ast::Scalar = Scalar::Complex(Complex::Complex(
        IntFloat::Integer(301),
        IntFloat::Float((21, 0.89)),
    ));
    let expected: Result<(&str, ast::Scalar), nom::Err<nom::error::Error<&str>>> = Ok(("", output));
    let actual = parse_complex(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_complex_float_float() {
    let string = "35.232J20.239";
    let input = &reverse_line(string);
    let output: ast::Scalar = Scalar::Complex(Complex::Complex(
        IntFloat::Float((35, 0.232)),
        IntFloat::Float((20, 0.239)),
    ));
    let expected: Result<(&str, ast::Scalar), nom::Err<nom::error::Error<&str>>> = Ok(("", output));
    let actual = parse_complex(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_complex_neg_float_float() {
    let string = "¯35.232J20.2112";
    let input = &reverse_line(string);
    let output: ast::Scalar = Scalar::Complex(Complex::Complex(
        IntFloat::Float((-35, 0.232)),
        IntFloat::Float((20, 0.2112)),
    ));
    let expected: Result<(&str, ast::Scalar), nom::Err<nom::error::Error<&str>>> = Ok(("", output));
    let actual = parse_complex(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_complex_neg_int_int() {
    let string = "¯35J20";
    let input = &reverse_line(string);
    let output: ast::Scalar = Scalar::Complex(Complex::Complex(
        IntFloat::Integer(-35),
        IntFloat::Integer(20),
    ));
    let expected: Result<(&str, ast::Scalar), nom::Err<nom::error::Error<&str>>> = Ok(("", output));
    let actual = parse_complex(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_id() {
    let string = "str1";
    let input = &reverse_line(string);
    let output: ast::Scalar = Scalar::Identifier(Identifier("str1".to_string()));
    let expected: Result<(&str, ast::Scalar), nom::Err<nom::error::Error<&str>>> = Ok(("", output));
    let actual = parse_id(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_id_error() {
    let string = "2str1";
    let input = &reverse_line(string);
    let expected: Result<(&str, ast::Scalar), nom::Err<nom::error::Error<&str>>> =
        Err(nom::Err::Failure(Error {
            input: "1rts2",
            code: nom::error::ErrorKind::Alpha,
        }));
    let actual = parse_id(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_id_error_panic() {
    let string: &str = "-";
    let input = &reverse_line(string);
    let expected: Result<(&str, ast::Scalar), nom::Err<nom::error::Error<&str>>> =
        Err(nom::Err::Failure(Error {
            input: "-",
            code: nom::error::ErrorKind::Alpha,
        }));
    let actual = parse_id(input);
    // println!("Actual: {:?}", actual);
    // println!("Expected: {:?}", expected);
    assert_eq!(actual, expected);
}
