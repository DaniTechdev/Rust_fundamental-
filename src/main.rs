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