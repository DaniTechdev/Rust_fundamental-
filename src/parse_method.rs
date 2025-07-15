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