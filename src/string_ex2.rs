
//Fill in the blanks

fn main(){
    let mut s:String =String::from("hello, world");

    // let slice1:&str = &s; //&String-> &str //or
    let slice1:&str = s.as_str(); //

    assert_eq!(slice1, "hello, world");


    let slice2: &str = &s[0..5];
    assert_eq!(slice2,"hello");

    let slice3: &mut String = &mut s;
    //we cam simple take ownership of s since we dont use it after the  assignment
    // let mut slice3 :String = s;
    slice3.push('!'); 
    assert_eq!(slice3,"hello, world!");

    println!("success");
}