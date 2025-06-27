fn main(){
    //use as many approaches as you can to make the code work
    // let x: String = String::from("Hello world");
    // let y: String = x.clone(); // Cloning the string to avoid ownership issues
    // println!("{},{}", x,y);


    //Don't modify the code in main

    // let s1:String = String::from("Hello, rust");
    // let s2 = take_ownership(s1);

    // println!("{}", s2);


    // let s = give_ownership();
    // println!("{}",s);


    // let s = String::from("hello, rust");

    // print_str(s.clone());

    // println!("{}", s);


    // let x: (i32, i32, (), &str) = (1,2,(),"hello");
    // let y = x.clone();

    // println!("{:?}, {:?}", x,y);

    //Mutability can be changed when ownership is transfered
    // let s= String::from("Hellow,");

    //Modify this line only
    // let mut s1: String = s;

    // s1.push_str("world");

    // println!("success");


    // let x: Box<i32> =Box::new(5); //Box is a smart pointer that allows you to allocate a stack variable to heap

    // let mut y: Box<i32> = Box::new(1); //implement this line ,don't change other line;
    // *y = 4;

    // assert_eq!(*x,5); //since x and y are just holding addresses(pointers), we will access their values using * in the front.
    // println!("success");


// #[derive(Debug)]

// struct Person{
//     name:String,
//     age:Box<u8>,
// }


// let person: Person = Person{
//     name:String::from("Allice"),
//     age:Box::new(20),
// };

// //`name` is moved out of person, but `age` is referenced
// let Person {name,ref age}= person;


// println!("The person's aage is {}", age);
// println!("The peron's name is {}",name);


//Erro! borrow of partially moved value: `person` partial move occurs
//println!("the person struct is {:?}", perosn);

//`person` cannot be used but `person.age` can be used as it is not moved


// let t: (String,String) = (String::from("hellow"), String::from("world"));

// let _s:String= t.0;

// //modify this line only , don't use `_s`
// println!("{:?}", t.1)


// let t:(String,String) = (String::from("hello"), String::from("World"));

// //file the blanks

// let (s1,s2) = t.clone();

// println!("{:?},{:?},{:?}",s1,s2,t) //-> "Hello", "world", ("hello","world");


// let s1 = String::from("hello");

// let len =  calculate_length(&s1);

// println!("The length of '{}' is {}.", s1,len);




}

// fn calculate_length(s:&String)-> usize{
//     s.len()
// }

// fn print_str(s:String){
//     println!("{}",s);
    
// }
//only modify the code below

// fn give_ownership()-> String{
//     let s: String =  String::from("Hellow, world");
//     //convert string to vec
//     // let _s = s.into_bytes();//this converts the string to bytes in vector form and it consumes the string being converted

//     let _s =  s.as_bytes(); //this converts the string to bytes in vector form but does not consume the string 

//     s
// }

//only modify the code below

// fn take_ownership(s:String) ->String{
//     s
// }