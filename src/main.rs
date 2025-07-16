//Fill in the blanks and fix the errors
use std::num::ParseIntError;

fn multiply(nl_str:&str, n2_str:&str)-> Result<i32, ParseIntError>{
    let n1: Result<i32, ParseIntError> = nl_str.parse::<i32>(); //Ok(10)
    let n2: Result<i32, ParseIntError> = n2_str.parse::<i32>(); //Ok(2)

    Ok(n1.unwrap()*n2.unwrap()) //Ok(20)
}

fn main(){
    let result: Result<i32, ParseIntError> = multiply("10", "2");

    assert_eq!(result,Ok(20));

    let result: Result<i32, ParseIntError> = multiply("4`", "2");
    assert_eq!(result.unwrap(),8);

    println!("Success");
}