extern crate nom;
use std::collections::HashMap;
use nom::{
  branch::alt, bytes::complete::{tag, tag_no_case, take_till, take_until, take_while, take_while_m_n}, character::{complete::{digit1, newline}, is_space, streaming::{alphanumeric0, char, line_ending}}, combinator::{map, map_res, opt, peek}, error::{convert_error, Error, VerboseError}, sequence::{preceded, separated_pair, terminated, tuple}, Err, IResult
};
use APL_convertor::ast::*;


fn parse_assign(input: &str) -> IResult<&str,(&str,&str)>  {
  separated_pair(tag("str"), char('←'), tag("'Hello world'"))(input)
}



fn parse_comment_content(input:&str) -> IResult<&str, &str> {
  preceded(tag("⍝"),take_until( "\n"))(input)
}

fn parse_comment(input: &str) -> IResult<&str,&str>   {
  parse_comment_content(input)
}

fn parse_line(input: &str) -> IResult<&str,&str>   {
  Ok((input,&"str"))
}


fn parse_apl(input: &str) -> IResult<&str, Vec<&str>>  {
  let mut vec = Vec::new();
  if let Ok((remainder,output)) = alt((
    parse_comment,
    parse_comment,
  ) 
  )(input) {
    vec.push(output);
    return Ok((remainder,vec));
  } else {
    return Ok(("",vec));
  };
}

fn parse_lines(input: Vec<&str>) -> IResult<&str, Vec<Vec<Stmt>>> {
  let mut vec_lines = Vec::new();
  let mut vec_line = Vec::new();
  let stmt = Stmt::Vector(Vector::Scalar(Scalar::IntFloat(IntFloat::Integer(2))));
  vec_line.push(stmt);
  vec_lines.push(vec_line);
  Ok(("", vec_lines))
}

fn reverse(s: &str) -> String {
  s.chars().rev().collect()
}

fn split_str_reverse_lines(s: &str) -> Vec<String> {
// Split the string into lines
  let vec_lines: Vec<String> = s.lines()
  .map(|line| reverse(line))
  .collect();
  // let mut vec_lines = Vec::new();
  // vec_lines.push("something");
   // Print the reversed lines
   return vec_lines;
}

fn reverse_line(input: &str) -> String {
  reverse(input)
}

use std::error::Error as StdError;
fn main() -> Result<(), Box<dyn StdError>>  {
  // println!("{}", SFun::Assign);
  // println!("{}", SFun::Conjugate);
  // println!("{}", SFun::Plus);
  // println!("{}", SFun::Negate);
  // println!("{}", SFun::Minus);
  // println!("{}", SFun::Direction);
  // println!("{}", SFun::Times);
  // println!("{}", SFun::Reciprocal);
  // println!("{}", SFun::Divide);
  // println!("{}", SFun::Exponential);
  // println!("{}", SFun::Power);
  // println!("{}", SFun::NaturalLogarithm);
  // println!("{}", SFun::Logarithm);
  // println!("{}", SFun::Comment);
  let code = "⍝testapl
  str ← 'Hello world'
  str";

  

  // let code = "⍝testapl\n";
  match parse_apl(code) {
    Ok((remainder,output)) => {
      println!("Remainder: {}",remainder);
      println!("Output: {:?}",output);
      
    },
    Err(error) => {
      println!("{}",error);
    }
  }
  
  let vec_lines = split_str_reverse_lines(code) ;

  let mut ref_vec_lines: Vec<&str> = Vec::new(); 
  for line in vec_lines.iter() {
    ref_vec_lines.push(line.as_str());
  }
  // for line in &vec_lines {
  //   println!("{}", line);
  // }


  match parse_lines(ref_vec_lines) {
    Ok((remainder,output)) => {
      println!("Remainder: {}",remainder);
      println!("Output: {:?}", output);
    },
    Err(error) => {
      println!("{}",error);
    }
  }
  // if let Ok((remainder,output)) = parse_apl(code) {
  //   println!("{}",remainder);
  //   println!("{}",output);
  // } else if let Error(error) = parse_apl(code) {
    
  // };
  return Ok(());
}


//// SCALAR PARSERS

fn parse_str_to_int(input: &str) -> IResult<&str, i64> {
  // let res = alt((terminated(digit1,tag("¯")),digit1))(input);
  match parse_negative(input) {
    Ok((remainder,(output,boolean))) => {
      let rev: String = reverse(output);
      let res = rev.parse::<i64>();
      match res {
        Ok(int) => match boolean {
          true => Ok((remainder,-int)),
          false => Ok((remainder,int)),
        }
        Err(_error) => Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Digit)))
      }
    }
    Err(error) => Err(error)
  }
}
fn parse_str_to_int1(input: &str) -> IResult<&str, i64> {
  match digit1(input) {
    Ok((remainder,output)) => {
      let rev: String = reverse(output);
      let res = rev.parse::<i64>();
      match res {
        Ok(int) => Ok((remainder,int)),
        Err(_error) => Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Digit)))
      }
    }
    Err(error) => Err(error)
  }
}

// use std::num::ParseFloatError;

fn parse_str_to_float(input: &str) -> IResult<&str, f64> {
  let zero: String = "0.".to_string();
  let res = alt((terminated(digit1,tag("¯")),digit1))(input);
  match res {
    Ok((remainder,output)) => {
      let rev: String = reverse(output);
      let combined: &str = & (zero + &rev);
      let res = combined.parse::<f64>();
      match res {
        Ok(float) => Ok((remainder,float)),
        Err(_error) => Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Float)))
      }
    }
    Err(error) => Err(error)
  }
}

/// Format Imaginary ["j","J"] Real
fn parse_complex(input: &str) ->  IResult<&str,APL_convertor::ast::Complex> {
  match separated_pair(parse_intfloat, alt((tag("j"),tag("J"))), parse_intfloat)(input) {
    Ok((remainder, (first, second))) => {
      // real part is first, i part is second
      Ok((remainder, Complex::Complex(second,first)))
    },
    Err(error) => Err(error)
  }
}

fn parse_float(input:&str) -> IResult<&str, APL_convertor::ast::IntFloat> {
  match separated_pair(parse_str_to_float, tag("."), parse_str_to_int)(input) {
    Ok((remainder, (first, second))) => {
      Ok((remainder, (IntFloat::Float((second, first)))))
    },
    Err(error) => Err(error)
  }
}

fn parse_int(input:&str) -> IResult<&str, APL_convertor::ast::IntFloat> {
  match parse_str_to_int(input) {
    Ok((remainder, ( second))) => {
      Ok((remainder, (IntFloat::Integer(second))))
    },
    Err(error) => Err(error)
  }
}


/// bool = true => negative
fn parse_negative(input : &str) -> IResult<&str, (&str,bool)> {
  let res : IResult<&str,&str> = terminated(digit1,tag("¯"))(input);
  match res {
    Ok((remainder,output)) => return Ok((remainder,(output,true))),
    // todo combine errors
    Err(_error) => {
      let res : IResult<&str,&str> = digit1(input);
      match res {
        Ok((remainder,output)) => return Ok((remainder,(output,false))),
        Err(error2) => Err(error2)
      }
    }
  }
}


fn parse_intfloat(input: &str) ->  IResult<&str,APL_convertor::ast::IntFloat> {

  let res: IResult<&str,char> = peek(char('-'))(input);
  alt((parse_float,
      parse_int),
  ) (input)
}

// TESTS

#[test]
fn test_parse_float() {
  // assuming lines are reversed?
  let string = "312.23311";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::IntFloat = IntFloat::Float((312,0.23311));
  let expected: Result<(&str, APL_convertor::ast::IntFloat), nom::error::Error<&str>> = Ok(("",output));
  let actual = parse_intfloat(input);
  println!("Actual: {:?}", actual);
  println!("Expected: {:?}", expected);
  // assert_eq!(actual,expected);
}

#[test]
fn test_parse_int() {
  // assuming lines are reversed?
  let string = "312";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::IntFloat = IntFloat::Integer(312);
  let expected: Result<(&str, APL_convertor::ast::IntFloat), nom::error::Error<&str>> = Ok(("",output));
  let actual = parse_intfloat(input);
  println!("Actual: {:?}", actual);
  println!("Expected: {:?}", expected);
  // assert_eq!(actual,expected);
}


#[test]
fn test_parse_complex_int_int() {
  let string = "31J23";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Complex= Complex::Complex(IntFloat::Integer(31),IntFloat::Integer(23));
  let expected: Result<(&str, APL_convertor::ast::Complex), nom::error::Error<&str>> = Ok(("",output));
  let actual = parse_complex(input);
  println!("Actual: {:?}", actual);
  println!("Expected: {:?}", expected);
  // assert_eq!(actual,expected);
}

#[test]
fn test_parse_complex_float_int() {
  let string = "31.23J223";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Complex= Complex::Complex(IntFloat::Float((31,0.23)),IntFloat::Integer(223));
  let expected: Result<(&str, APL_convertor::ast::Complex), nom::error::Error<&str>> = Ok(("",output));
  let actual = parse_complex(input);
  println!("Actual: {:?}", actual);
  println!("Expected: {:?}", expected);
  // assert_eq!(actual,expected);
}

#[test]
fn test_parse_complex_int_float() {
  let string = "301J21.89";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Complex= Complex::Complex(IntFloat::Integer(301),IntFloat::Float((21,0.89)));
  let expected: Result<(&str, APL_convertor::ast::Complex), nom::error::Error<&str>> = Ok(("",output));
  let actual = parse_complex(input);
  println!("Actual: {:?}", actual);
  println!("Expected: {:?}", expected);
  // assert_eq!(actual,expected);
}

#[test]
fn test_parse_complex_float_float() {
  let string = "35.232J20.239";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Complex= Complex::Complex(IntFloat::Float((35,0.232)),IntFloat::Float((20,0.239)));
  let expected: Result<(&str, APL_convertor::ast::Complex), nom::error::Error<&str>> = Ok(("",output));
  let actual = parse_complex(input);
  println!("Actual: {:?}", actual);
  println!("Expected: {:?}", expected);
  // assert_eq!(actual,expected);
}

#[test]
fn test_parse_complex_neg_float_float() {
  let string = "¯35.232J20.2112";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Complex= Complex::Complex(IntFloat::Float((-35,0.232)),IntFloat::Float((20,0.2112)));
  let expected: Result<(&str, APL_convertor::ast::Complex), nom::error::Error<&str>> = Ok(("",output));
  let actual = parse_complex(input);
  println!("Actual: {:?}", actual);
  println!("Expected: {:?}", expected);
  // assert_eq!(actual,expected);
}


#[test]
fn test_parse_complex_neg_int_int() {
  let string = "¯35J20";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Complex= Complex::Complex(IntFloat::Integer(-35),IntFloat::Integer(20));
  let expected: Result<(&str, APL_convertor::ast::Complex), nom::error::Error<&str>> = Ok(("",output));
  let actual = parse_complex(input);
  println!("Actual: {:?}", actual);
  println!("Expected: {:?}", expected);
  // assert_eq!(actual,expected);
}