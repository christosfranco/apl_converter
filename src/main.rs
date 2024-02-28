extern crate nom;
use std::collections::HashMap;
use nom::{
  branch::alt, bytes::complete::{tag, tag_no_case, take_till, take_until, take_while, take_while_m_n}, character::{complete::{digit1, newline}, is_space, streaming::{alphanumeric0, char, line_ending}}, combinator::{map, map_res}, error::{convert_error, Error, VerboseError}, sequence::{preceded, separated_pair, terminated, tuple}, Err, IResult
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

fn parse_str_to_int(input: &str) -> IResult<&str, i64> {
  map_res(digit1, str::parse::<i64>)(input)
}
use std::num::ParseFloatError;
fn parse_str_to_float(input: &str) -> IResult<&str, f64> {
  let zero: String = "0.".to_string();
  match digit1(input) {
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


fn parse_j(input:&str) -> IResult<&str, &str> {
  alt((
    tag("j"),
    tag("J"),
  )
  )(input)
}

/// Format Imaginary J Real
fn parse_complex(input: &str) ->  IResult<&str,APL_convertor::ast::Complex> {
  match separated_pair(parse_str_to_int, parse_j, parse_str_to_int)(input) {
    Ok((remainder, (first, second))) => {
      // real part is first, i part is second
      Ok((remainder, Complex::Complex(IntFloat::Integer(second),IntFloat::Integer(first))))
    },
    Err(error) => Err(error)
  }
}

fn parse_intfloat(input: &str) ->  IResult<&str,APL_convertor::ast::IntFloat> {
  match separated_pair(parse_str_to_float, tag("."), parse_str_to_int)(input) {
    Ok((remainder, (first, second))) => {
      let first2 = first ;
      let second2 = second;
      Ok((remainder, (IntFloat::Float(first+second as f64))))
    },
    Err(error) => Err(error)
  }
}

// TESTS

#[test]
fn test_parse_float() {
  // assuming lines are reversed?
  let string = "3.2";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::IntFloat = IntFloat::Float(3.2);
  let expected: Result<(&str, APL_convertor::ast::IntFloat), nom::error::Error<&str>> = Ok(("",output));
  let actual = parse_intfloat(input);
  println!("Actual: {:?}", actual);
  println!("Expected: {:?}", expected);
  // assert_eq!(actual,expected);
}


#[test]
fn test_parse_complex() {
  let string = "3J2";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Complex= Complex::Complex(IntFloat::Integer(3),IntFloat::Integer(2));
  let expected: Result<(&str, APL_convertor::ast::Complex), nom::error::Error<&str>> = Ok(("",output));
  let actual = parse_complex(input);
  println!("Actual: {:?}", actual);
  println!("Expected: {:?}", expected);
  // assert_eq!(actual,expected);
}

#[test]
fn test_parse_color() {
  assert_eq!(hex_color("#2F14DF"), Ok(("", Color {
    red: 47,
    green: 20,
    blue: 223,
  })));
}


// PARSE COLOR
#[derive(Debug,PartialEq)]
pub struct Color {
  pub red:   u8,
  pub green: u8,
  pub blue:  u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
  map_res(
    take_while_m_n(2, 2, is_hex_digit),
    from_hex
  )(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
  let (input, _) = tag("#")(input)?;
  let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

  Ok((input, Color { red, green, blue }))
}