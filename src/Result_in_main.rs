use core::num;
use std::num::ParseIntError;

fn main() -> Result<(),ParseIntError>{
    let number_str:&str = "10";
    // let number: i32 = match number_str.parse::<i32>(){
    //     Ok(number)=>number,
    //     Err(e)=> return Err(e),
    // };

    let number1: i32 = number_str.parse::<i32>()?;
    println!("{}", number1);
    Ok(())
}