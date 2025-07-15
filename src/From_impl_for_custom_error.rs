use std::fs;
use std::io;
use std::num;


enum CliError{
    IoError(io::Error),
    ParseError(num::ParseIntError)
}


impl From<io::Error> for CliError{
    //implement from method
    //   fn from(value: T) -> Self;
      fn from(e:io::Error) -> Self{
        CliError::IoError(e)
      }
}

impl From<num::ParseIntError> for CliError{
    //implement method here
    //   fn from(value: T) -> Self;
       fn from(e: num::ParseIntError) -> Self{
        CliError::ParseError(e)
       }
}

fn open_and_parse_file(file_name:&str)-> Result<i32,CliError>{
    //? automatically converts io::Error to CliError
    let contents: String = fs::read_to_string(&file_name)?;
    //num::ParseIntError -> CliError
    let num:i32 = contents.trim().parse()?;
    Ok(num)
}


fn main(){
    println!("Success");
}