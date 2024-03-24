
use apl_converter::generator::ast::{IntFloat, Scalar, Stmt, StmtLst, Vector};



use apl_converter::generator::generator_lib::{generate_py,string_to_stmtlst};
use nom::error::convert_error;
use nom::IResult;
use std::error::Error as StdError;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

// use parser::parser_lib ;

// use generator lib
// pub mod generator_lib;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "APL generator",
    about = "APL generator is a generator tool to convert am APL Abstract syntax tree to code."
)]

pub struct ProgArgs {
    /// Code file. Example: examples/hello_world.apl. If not set will expect `input_code`
    #[structopt(short, long)]
    file: Option<PathBuf>,

    /// Input code string. Use this if `file` is not set.
    #[structopt(short, long)]
    input: Option<String>,


    /// Which language to convert to.
    #[structopt(short, long, default_value = "python3")]
    language: String,
}

fn read_file_contents(path: String) -> Result<String, Box<dyn StdError>> {
    // Read the file contents into a String
    let contents = fs::read_to_string(path)?;

    Ok(contents)
}



fn main() -> Result<(), Box<dyn StdError>> {
    let args = ProgArgs::from_args();
    let code = args.input;
    let file = args.file;

    let _language = args.language;
    // TODO return Err instead of Ok if error happens with appropiate new error type
    match (code, file) {
        (Some(_), Some(_)) => eprintln!("Cannot give both code and file as input"),
        (None, None) => eprintln!("Either code or file expected as input"),
        (None, Some(file)) => {
            if let Some(file) = file.to_str() {
                match read_file_contents(file.to_string()) {
                    // VALID FILE INPUT
                    Ok(contents) => {
                        // TODO return result here , instead of printing
                        if let Ok(stmtlst) = string_to_stmtlst(contents) {
                            match  generate_py(stmtlst) {
                                Ok(res) => {
                                    println!("Result: {:?}", res);
                                },
                                Err(error) => {
                                    eprintln!("Could not generate python from Statement List with error: {}", error);
                                    return Err(error);
                                }
                            }
                        } else {
                            eprintln!("Could not generate Statement List form input string ");
                        }                        
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
            // TODO return result here , instead of printing
            println!("Result: {:?}", code);
        }
    }
    Ok(())
}




#[test]
fn test_parse_matrix_basic() {
    // let string = "7";
    // // reverse the input as references to adhere to the borrowchecker
    // let rev_string = parser_lib::split_str_reverse_lines(string);

    // let mut input: Vec<&str> = Vec::new();
    // for line in rev_string.iter() {
    //     input.push(line.as_str());
    // }
    // let vector0: Vector = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(7)));
    // let vec10: Vector = Vector::Scalar(None, Scalar::IntFloat(IntFloat::Integer(7)));
    let scalar: Scalar = Scalar::IntFloat(IntFloat::Integer(7));
    let vector: Vector = Vector::Scalar(None, scalar);
    let input: StmtLst = StmtLst::Statement(None, Stmt::LeftStmt(vector, None));
    
    let expected_string = "7";
    println!("{:?}",input);
    let actual_string = generate_py(input);

    match actual_string {
        Ok(string) => {
            assert_eq!((string), expected_string);
        },
        Err(_e) => {
            assert_eq!(1,2);
        }
    }
    
    // println!("Actual: {:?}", actual_vec);
    // println!("Expected: {:?}", expected_vec);

}

