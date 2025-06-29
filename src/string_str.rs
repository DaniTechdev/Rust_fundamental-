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


//oppostite to the soldom using of str, &str and string are used everywhere!


//fix error with at least two solutions
// let s:&str = "hello, world";
// greetings(s);

// let s:String = String::from("hello, world");
// greetings(s);


// let s:&str = "hello, world";
// greetings(s.to_string()); //converted a string slice to a String


// let s:&str = "hello, world";
// greetings(String::from(s));// converted a stringslice to a String


//We use String::from or to_string to convert from &str to String



//use two approaches to fix the error and without adding a new line

// let s = "hello, world".to_string();
// let s1:String = s;

// let s:String = "hello, world".to_string();
// // let s1:&str = &s; //ref to a String will convet to string slice &str
// let s1:&str = s.as_str();

// println!("success");


//You can use escape to write bytes by their hexadecimal values

//fill the blank  below to show "I'm writing Rust"

// let byte_escape:&str = "I'm writing Ru\x73\x74";
// println!("What are you doing \x3f ( \\x3f means ?) {}", byte_escape);

// //...Or Unicode code points
// let unicode_codepoint = "\u{211D}";
// let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

// println!("Unicode character {} (U+211D) is called {}",
// unicode_codepoint,character_name);

// let long_string =  r"String literals 
// can span multiple line.
// The linebreak and indentaation here \ can be escaped too!";

// println!("{}", long_string)


//You can't use index to access a char in a string , but you can use slice &s1[start..end];




// let s1: String = String::from("h1,⛳⛳");
// let h: &str = &s1[0..1]; //Modify this line to fix the error, tips: `h` only takes 1 byte
// assert_eq!(h, "h");

// let h1 = &s1[3..6]; //Modify this line to fix the error, tips: ⛳ takes 3 bytes
// assert_eq!(h1,"⛳");

// println!("{}",h1);
// println!("success");



//fill the blank to print each char in "⛳⛳⛳⛳⛳"

// for c in "⛳⛳⛳⛳⛳".chars(){ //chars() is a method that can put the strings into iterable characters

//     println!("{}", c)
// }
}


// fn greetings(s:String){
//     println!("{}",s);
// }


// fn greetings(s:&str){
//     println!("{}",s);
// }