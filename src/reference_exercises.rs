fn main(){
    // //exercise 1
    // let x: i32 = 5;
    // //fill the blank
    // let p: &i32 = &x;
    // println!("the memory address of x is {:p}", p); //One possible output: 0x16fa3ac


    // //exercise 2
    // let x: i32 = 5;
    // let y: &i32 = &x; //y here doesn't hold the value 5, it holds the memory address of 5
    // //modify this line only
    // assert_eq!(5,*y); //to access the value which y reference holds we have to dereference using *
    // println!("success");

    //exercise 3

    // let mut s: String = String::from("hello,");

    // borrow_object(&s);
    // println!("success");


    //exercise 3

    // let mut s: String = String::from("hello,");
    // push_str(&mut s);
    // println!("success");


    //exercise 4
    // let mut s: String = String::from("hello,");
    // //fill the blank to make it work
    // let p: &mut String = &mut s;

    // p.push_str("World");

    // println!("success");


    //exercie 5


    //ref can be used to take references to a value similar  to &

    // let c: char  = 'a';

    // let r1: &char = &c;
    // //fill the blank, don't change other code
    // let ref r2 = c;

    // assert_eq!(*r1, *r2);

    // //check the equality of the two address strings
    // assert_eq!(get_addr(r1), get_addr(r2));



    //Remove something to make it work
    //Don't remove a whole line!
    // let mut s: String = String::from("hello");

    // let r1 = &s;
    // let r2 = & s;

    // println!("{},{}", r1,r2);

    // println!("Success");
    // println!("s: {}", s);


    // //fix error by modifying this line
    // let  mut s = String::from("hello, ");

    // borrow_object(&mut s);

    // println!("success");


    //we can't pass mutable references to a function whose original varible is immutable
    //but it caan work the other way around as shown below

    // let mut s: String = String::from("hello, ");

    // borrow_object(&s);

    // s.push_str("world");
    
    // print!("success!");


    // //Comment one line to make it work

    // let mut s: String = String::from("hello, ");

    // let r1: &mut String = &mut s;
    // r1.push_str("World");
    
    // let r2: &mut String = &mut s;
    // r2.push_str("!");

    // // println!("{}", r1);


    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    //add one line below to make a compiler error: cannot borrow `s` aa mutable more than once at a time

    println!("cannot borrow `s` aa mutable more than once at a time");
    //You can't use r1 and r2 at the same time

    println!("{}, {}", r1, r2);
    



}

// fn borrow_object(s:& String){}


// fn get_addr(r:&char)-> String{
//     format!("{:p}",r) //{:p} basically gets the pointer 
//     //fomrat! returns what ever the function waants tto return unlike the println
// }


// fn push_str(s:&mut String){
//     s.push_str("World");
// }


// fn borrow_object(s:&String){}