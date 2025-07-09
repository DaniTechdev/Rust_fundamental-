//Fill in the blanks and Fix errors
//(1) Don't use `to_string`
//(2) Don't add/remove any code line.

fn main(){
    let mut s:String = String::from("hello,");

    s.push_str("world" );

    s.push('!'); //note: 


    move_ownership(s.clone()); // we clone the string to pass ownership
    // to the function without losing it in the main function
    // s is still valid here

    assert_eq!(s,"hello,world!");


}

fn move_ownership(s:String){
    println!("ownership of {} is moved here!" ,s);
}