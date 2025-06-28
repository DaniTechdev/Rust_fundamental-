fn main(){
    //String

    //The type of string literal "hello, world" is &str, eg 
    //let s:&str = "hellow , world".

    //Str and &str 

    //1) We can't type str in normal ways, but we can use &str.

    //fix error without adding new line

    // let s : &str  = "Hello, world";

    // println!("success");


    // let s:Box<str> = "hello, world".into(); //into method will conver a type into the type we are annotating there

    // greetings(&s);

    //or

    // let s:&str = "hello, world".into(); //into method will conver a type into the type we are annotating there

    // greetings(&s);


    //String type is defined in std and stored as a vector of bytes(Vec), but guarranteeed to alwaays
    //be valid UTF-8 sequence.String is heap allocated , growable and not null terminated



    // //fill the blank

    // let mut s: String =String::from("");
    // s.push_str("hello, world");

    // s.push('!');

    // assert_eq!(s, "hello, world!");

    // println!("{}",s);
    // println!("success");


    // //fix all errors without adding newline

    // let mut s: String = String::from("hello");

    // s.push(',');
    // s.push_str(" world");
    // // s += "!";
    // s = s + "!";

    // println!("{}",s);
    

    //replace can be used to replace substring

    // let  s: String =  String::from("I like dogs");

    // //Allocate new memory and store the modified string there
    // let s1: String = s.replace("dogs", "cats");

    // assert_eq!(s1, "I like cats");
    // println!("{},{}",s,s1);
    
    // println!("success");


    //More  String methods can be found under String  module documentation

    //You can only concat a string with &str, and String's ownership can be moved to 
    //another variable, but you can't concat a String with another String directly.

    //fix error without adding new line
    // let s1 = String::from("hello,");
    // let  s2: &str = " world!";
    // let s3 = s1 + s2;


    // assert_eq!(s3 , "hello, world!");
    // println!("{}", s3);

    //or

    // let s1 = String::from("hello,");
    // let  s2: String= String::from(" world!");
    // let s3: String = s1 + s2.as_str(); //as_str() converts String to &str


    // assert_eq!(s3 , "hello, world!");
    // println!("{}", s3);


//or 

// let s1 = String::from("hello,");
// let  s2: String= String::from(" world!");
// let s3: String = s1 + &s2; // a reference to a string can convert the string to &str


// assert_eq!(s3 , "hello, world!");
// println!("{}", s3);


//oppsio

}


// fn greetings(s:&str){
//     println!("{}",s);
// }