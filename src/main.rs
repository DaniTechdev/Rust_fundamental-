// use std::any::type_name_of_val;

// use std::ops::RangeInclusive;
// use std::ops::{Range, RangeInclusive};


#[allow(unused_variables)]

fn main(){
    // let x:i32 = 40;
    // let  y =5;

    // let   y=x;

    // let z:i32 = 10;

    // println!("success");
    // println!("{y}");

    //assigning interger type directly

    // let v:u16 = 38_u8 as u16;

    // println!("success");



    //quiz
    //modify assert_eq! t make it work

    // let x:u32 = 5;
    // assert_eq!("u32".to_string(), type_of(&x));

    // println!("success");



    //fill the blanks to make the code work
    // assert_eq!(i8::MAX,127);
    // assert_eq!(u8::MAX,255);
  
    // println!("success");


    //fix error and panics to make it work

    // let v1 = 251_u16 +8_u16;
    // let v2:u16=  u16::checked_add(251,8 ).unwrap();

    // println!("{},{}", v1,v2);


    //modify assert to make it work

    // let v = 1_024 + 0xff + 0o77 + 0b1111_1111; 
    // assert!(v == 1597);


    //Fill the blank to make it work

    // let x = 1_000.000_1; //?
    // let y:f32 = 0.12; //f32
    // let z = 0.01_f64; //f64 

    // assert_eq!(type_of(&x),"&f64".to_string());

    // println!("Success");

    //Make it work in two distint ways

    // assert!(0.1_f32 + 0.2_f32 == 0.3_f32); // 0.1 + 0.2 = 0.30000000000002 so use float type to ocnver them to exact
    // //or
    // assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);

    // print!("Success");



    //Tow goals: 1 Modify assert! to make it work 2. Make printlin! output: 97 to 122;

    //  let mut sum:i32 = 0;
    //  for i in -3..2 { //note end points in the range right side is excluded in Rust e.g 2
    //     sum +=i;
        
    //  }

    //  println!("sum is: {}", sum);

    //  assert!(sum == -5);

    //  for c in "a"..="z"{// in this case the right side of the range is included because we used the equality
    //     println!("{}", c)
    //  }

    //  for c in 'a'..='z' { // Note the single quotes for characters
    //     println!("{}", c as u8); // Convert character to u8 to print its ASCII value
    // }



     //Fill the planks


    //  assert_eq!((1..5), Range{start:1, end:5});
    //  assert_eq!((1..=5), RangeInclusive::new(1,5));

    //  println!("success");

    //Fill the blanks and fix the errors
    
    //Inter addition
    assert!(1_u32 + 2 == 3);

    //Integer subtraction
    assert!(1_i32 - 2i32 ==-1i32);

    assert!(1i8 - 2i8 == -1i8);

    assert!(3 * 50 == 150);//i32

    assert!(9.6 as f32 /3.2 as f32 == 3.0 as f32); //error! make it work

    assert!(24 % 5 == 4);
    println!("success");

    //short = circuting boolean logic
    assert!(true && false == false);
    // assert!(true );
    
}

  //Question
    //Get the type ofa variable in Rust, returm a string representation of the type 

    // fn type_of<T>(_: T) -> String {
    //    format!("{}", std::any::type_name:: <T>()) //returns string of any valu passed as parameter e.g "i32"
    // }

