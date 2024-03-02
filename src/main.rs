extern crate nom;
use std::collections::HashMap;
use nom::{
  branch::alt, bytes::complete::{tag, tag_no_case, take_till, take_until, take_while, take_while_m_n}, character::{complete::{alpha1, alphanumeric0, char, digit1, newline}, is_space}, combinator::{map, map_res, opt, peek, recognize}, error::{convert_error, Error, VerboseError}, sequence::{preceded, separated_pair, terminated, tuple}, Err, IResult
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
fn parse_complex(input: &str) ->  IResult<&str,APL_convertor::ast::Scalar> {
  match separated_pair(parse_intfloat, tag_no_case("j"), parse_intfloat)(input) {
    Ok((remainder, (first, second))) => {
      // real part is first, i part is second
      Ok((remainder, Scalar::Complex(Complex::Complex(second,first))))
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
    Ok((remainder, second)) => {
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
  let res = alt((parse_float,
      parse_int),
  ) (input);
  match res {
    Ok((remainder,output)) => Ok((remainder,(output))),
    Err(error) => Err(error)
  }
}

fn parse_id(input : &str) -> IResult<&str,APL_convertor::ast::Scalar > {
  let res: IResult<&str, &str> = recognize(alphanumeric0)(input); 
  match res {
    Ok((remainder,output)) => {
      match output.chars().last().unwrap_or(' ').is_alphabetic() {
        true => Ok((remainder,Scalar::Identifier(Identifier(reverse(output).to_string())))),
        false => Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Alpha)))
      }
    },
    Err(error) => Err(error)
  }
}

fn parse_scalar(input:&str) -> IResult<&str,APL_convertor::ast::Scalar> {
  let res = alt((parse_complex,parse_id))(input);
  match res {
    Ok((remainder,output)) => Ok((remainder, (output))),
    Err(error) => Err(error)
  }
}

/// end SCALAR PARSERS

// vector         ::= vector* ( scalar | ( LPARENS statement RPARENS ) )
// vector is 0 or many vectors // vector will can thus be a matrix

// #[derive(Debug)]
// pub enum Vector {
//     Multiple(Vec<Vector>, Scalar),
//     Scalar(Scalar),
//     Stmt(Box<Stmt>),
// }

fn parse_vector(input:&str) -> IResult<&str, APL_convertor::ast::Vector> {
  let vec = Vec::new();
  let res =   (parse_intfloat)(input);
  match res {
    Ok((remainder, output)) => Ok((remainder,Vector::Multiple(vec,Scalar::IntFloat(output)))),
    Err(error) => Err(error)
  }
}


// TESTS
#[test]
fn test_parse_float() {
  // assuming lines are reversed?
  let string = "312.23311";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::IntFloat = IntFloat::Float((312,0.23311));
  let expected: Result<(&str, APL_convertor::ast::IntFloat),nom::Err<nom::error::Error<&str>>> = Ok(("",output));
  let actual = parse_intfloat(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
   assert_eq!(actual,expected);
}

#[test]
fn test_parse_int() {
  // assuming lines are reversed?
  let string = "312";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::IntFloat = IntFloat::Integer(312);
  let expected: Result<(&str, APL_convertor::ast::IntFloat),nom::Err<nom::error::Error<&str>>> = Ok(("",output));
  let actual = parse_intfloat(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
   assert_eq!(actual,expected);
}


#[test]
fn test_parse_complex_int_int() {
  let string = "31J23";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Scalar=  Scalar::Complex ( Complex::Complex(IntFloat::Integer(31),IntFloat::Integer(23)) );
  let expected: Result<(&str, APL_convertor::ast::Scalar),nom::Err<nom::error::Error<&str>>> = Ok(("",output));
  let actual = parse_complex(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
   assert_eq!(actual,expected);
}

#[test]
fn test_parse_complex_float_int() {
  let string = "31.23J223";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Scalar=  Scalar::Complex( Complex::Complex(IntFloat::Float((31,0.23)),IntFloat::Integer(223)) );
  let expected: Result<(&str, APL_convertor::ast::Scalar),nom::Err<nom::error::Error<&str>>> = Ok(("",output));
  let actual = parse_complex(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
   assert_eq!(actual,expected);
}

#[test]
fn test_parse_complex_int_float() {
  let string = "301J21.89";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Scalar= Scalar::Complex( Complex::Complex(IntFloat::Integer(301),IntFloat::Float((21,0.89))) );
  let expected: Result<(&str, APL_convertor::ast::Scalar),nom::Err<nom::error::Error<&str>>> = Ok(("",output));
  let actual = parse_complex(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
   assert_eq!(actual,expected);
}

#[test]
fn test_parse_complex_float_float() {
  let string = "35.232J20.239";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Scalar=  Scalar::Complex( Complex::Complex(IntFloat::Float((35,0.232)),IntFloat::Float((20,0.239))) );
  let expected: Result<(&str, APL_convertor::ast::Scalar),nom::Err<nom::error::Error<&str>>> = Ok(("",output));
  let actual = parse_complex(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
   assert_eq!(actual,expected);
}

#[test]
fn test_parse_complex_neg_float_float() {
  let string = "¯35.232J20.2112";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Scalar= Scalar::Complex( Complex::Complex(IntFloat::Float((-35,0.232)),IntFloat::Float((20,0.2112))) );
  let expected: Result<(&str, APL_convertor::ast::Scalar),nom::Err<nom::error::Error<&str>>> = Ok(("",output));
  let actual = parse_complex(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
   assert_eq!(actual,expected);
}


#[test]
fn test_parse_complex_neg_int_int() {
  let string = "¯35J20";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Scalar= Scalar::Complex( Complex::Complex(IntFloat::Integer(-35),IntFloat::Integer(20)) );
  let expected: Result<(&str, APL_convertor::ast::Scalar),nom::Err<nom::error::Error<&str>>> = Ok(("",output));
  let actual = parse_complex(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
   assert_eq!(actual,expected);
}


#[test]
fn test_parse_id() {
  let string = "str1";
  let input = &reverse_line(string);
  let output : APL_convertor::ast::Scalar= Scalar::Identifier( Identifier("str1".to_string()));
  let expected: Result<(&str, APL_convertor::ast::Scalar), nom::Err<nom::error::Error<&str>>> = Ok(("",output));
  let actual = parse_id(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
  assert_eq!(actual,expected);
}


#[test]
fn test_parse_id_error() {
  let string = "2str1";
  let input = &reverse_line(string);
  // let output : APL_convertor::ast::Identifier= Identifier("str1".to_string());
  let expected : Result<(&str, APL_convertor::ast::Scalar), nom::Err<nom::error::Error<&str>>>= Err(nom::Err::Failure(Error { input: "1rts2", code: nom::error::ErrorKind::Alpha }));
  let actual = parse_id(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
   assert_eq!(actual,expected);
}

#[test]
fn test_parse_id_error_panic() {
  let string: &str = "-";
  let input = &reverse_line(string);
  let expected : Result<(&str, APL_convertor::ast::Scalar), nom::Err<nom::error::Error<&str>>>= Err(nom::Err::Failure(Error { input: "-", code: nom::error::ErrorKind::Alpha }));
  let actual = parse_id(input);
  // println!("Actual: {:?}", actual);
  // println!("Expected: {:?}", expected);
   assert_eq!(actual,expected);
}