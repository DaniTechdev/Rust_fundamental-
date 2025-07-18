use std::{num::ParseIntError, result};

//with the return type rewritten, we use pattern matching without `unwrap()`
//but it's so verbose...

fn multiply(n1_str:&str, n2_str:&str) -> Result<i32, ParseIntError>{
    match n1_str.parse::<i32>(){
        Ok(n1)=>{
            match n2_str.parse::<i32>() {
                Ok(n2)=> {
                    Ok(n1 * n2)
                },
                Err(e)=> Err(e)
            }
        },
        Err(e)=> Err(e),
    }
}


//Rewritting `multiply` to make it succinct
//You should use BOTH of `and_then` and `maap` here.

fn multiply1(n1_str:&str, n2_str:&str) -> Result<i32, ParseIntError>{
    n1_str.parse::<i32>().and_then(|n1| n2_str.parse::<i32>().map(|n2| n1*n2 ) )
}

fn print(result:Result<i32,ParseIntError>) {
    match result {
        Ok(n)=> println!("n is {} ", n),
        Err(e)=> println!("Error: {}",e)
        
    }
}

fn main() {
    //This still presents a reasonable answer

    let twenty = multiply1("10", "2");

    print(twenty); //Ok(20)

    //The following now provides a much more helpful error message.

    let tt: Result<i32, ParseIntError> = multiply("t", "2");

    print(tt);

    println!("Success");
}


use std::num::ParseIntError;

//Fill in the blank

type _Res<i32> = Result<i32,ParseIntError>_;

//use the above alias to refer to our specific `Result` type.
fn multiply(first_number_str:&str, second_number_str:&str) -> Res<i32>{
    first_number_str.parse::<i32>().and_then(|first_number_str: &str| {
        second_number_str.parse::<i32>().map(| second_number_str: &str| first_number_str * second_number_str )
    }  );
}

//Here , the alias again allows us to save some space

fn print(result: Res<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e)=> println!("Error: {}", e),
    }
}


fn main(){
    println!(multiply("10", "2"));
    println!(multiply("t", "2"));
}