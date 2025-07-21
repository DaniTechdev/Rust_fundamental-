// fn main(){
//     let s1 = "hello";
//     //Fill in the blank

//     let s:String = format!("{}, world!", s1);

//     assert_eq!(s,"hello, world!");

//     println!("{}", s);
// }

// fn main(){
//     //Fill in the blanks to make it print
//     //Hello world, I am sunface

//     print!("hello world, ");
//     println!("Iam");
//     println!("Sunface");



// // }
// #[derive(   Debug)]
// struct Structure(i32);

// fn main (){
//     //Type in the std and Rust have implemented the fmt::Debug traait
//     println!("{} months in a year.",12);

//     println!("Now {:?} will print", Structure(3));
// }

// #[derive(Debug)]

// struct  Person{
//     name:String,
//     age:u8
// }


// fn main(){
//     let person:Person = Person{name:"Sunface".to_string(),age:18};

// // Make it output
// // Person {
// //     name:"Sunface",
// //     age:18
// // }
//  println!("{:#?}", person);

// }


//manual implement of Debug in custom type


// #[derive(Debug)]


use std::fmt;
struct  Structure(i32);

// #[derive(Debug)]
struct  Deep(Structure); //Deep(Structure(i32))

use std::fmt::{write, Formatter};

impl std::fmt::Debug for Deep {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        write!(f, "{}",self.0.0)
    }
}
fn main(){
    //The problem with `derive` is there is no control over how
    //the result look. What if I want this to just show a `7`?

    // Make it prin: Now 7 will print!

    println!("Now {:?} will print!", Deep(Structure(7)));
}
