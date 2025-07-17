//To use `from_str` method, you need to introduce this trait into the current scope
use std::str::FromStr;

fn main(){
    let parsed:i32  = "5".parse().unwrap();
    let turb_parsed = "10".parse::<i32>().unwrap(); // this happed because from_str trait is implemented for i32
    let from_str = i32::from_str("20").unwrap(); // this happed because from_str trait is implemented for i32
    let sum = parsed + turb_parsed + from_str;
    assert_eq!(sum,35);

    println!("Success");
}


use std::num::ParseIntError;

//Implement multiply with ?
//Don't use unwrap here

fn multiply(n1_str:&str, n2_str:&str) -> Result<i32, ParseIntError>{
  let    n1: i32= n1_str.parse::<i32>()?; //Ok(3) -> 3
  let    n2: i32= n2_str.parse::<i32>()?; //Ok(2)-> 4


  Ok(n1 * n2) 


}

fn main(){
    assert_eq!(multiply("3", "4").unwrap(), 12); //Ok(12) -> 12
    println!("Success");
}