//`'a` must live longer than the function.
//Here , `&String::from("foo")` would creaate a `String` , followed by a
//reference.The the data is dropped upon exiting the scope, leaaving 
//aa referenc to invaalid dataa to be returned.


// Fix the error in three ways 

// fn invalid_output ()-> String {
//     String::from("foo")
// }


// fn main(){
//     let x: String = invalid_output();

//     println!("{}",x);
// }

// //best is to return the value and transfer ownership as above

// fn invalid_output<'a> ()-> &'a String {
//     &String::from("foo")
// }


// fn main(){
//     let x: &String = invalid_output();
// }


//2nd way is to return string literal. String literals have static lifetime as
//they are program inside the program itselg

// fn invalid_output ()-> &'static str {
//     "foo"
// }


// fn main(){
//     let x:&str  = invalid_output();

//     println!("{}",x);
// }

fn invalid_output<'a> (s:&'a String)-> &'a str { //&String -> &str
    s
}


fn main(){

    let s: String = String::from("foo");
    let x:&str  = invalid_output(&s);

    println!("{}",x);
}