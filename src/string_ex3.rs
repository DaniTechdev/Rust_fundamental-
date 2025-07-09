//Question: how many heap allocations are happening here?
//Your answer:2

fn main(){
    //create a String type based on `&str`
    //The type of string literals is `&str`

    let  s: String = String::from("hello, world!"); //1

    //create a slice point to String `s`
    let slice:&str = &s; //"hello, world!" //0

    //create a string type based on the recently created slice
    let s: String = slice.to_string(); //2

    assert_eq!(s, "hello, world!");

    println!("success");
}