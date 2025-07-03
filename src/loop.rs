use core::num;

fn main(){

    //Flow Control

    //Conditionals;
    //-if/else
    //-match

    //Loop
    //-for/while/loop
    //-continue/break


    // let n:i32 =-3;

    // if n < 0 {
    //     println!("{} is negative",n);

    // } else if n > 0 {
    //     println!("{} is positive",n);
    // }else{
    //     println!("{} is zero", n)
    // }
    

    // let n: i32 = 2005;

    // let big_n = 
    // if n<10 && n > -10 {
    //     println!(", and is a small number, increase ten-fold");

    //     10 * n 
    // } else {
    //     println!(", and is a big number, halve the number");

    //     n/2.0 as i32
    // };

    // println!("{} -> {}", n, big_n);



    //..............

    // for n in 1..100 { //modify this line to make the code work
    //     if n == 100 {
    //         panic!("Never let this run");
    //     } 
    // }

    // println!("Success"); 
    

    //fix the error without addiing or removing line;
    // let names: [String; 2] = [String::from("liming"), String::from("natochi")];

    // for name in &names {
    //     //Do  something  with name ...
    //     println!("{}", name);
    // };

    // println!("{:?}", names);

    // let numbers: [i32; 3] = [1,2,3];

    // //The element in numbers are copy, so there is no move here
    // for n in numbers {
    //     //Do somthing with name ...
    //     println!("{}", n);
    // }

    // println!("{:?}", numbers);


    // let a: [i32; 4] = [4,3,2,1];

    // //Iterate the indexing and value in "a"

    // for (i,v) in a.iter().enumerate() {
    //     println!("The {}th element {}", i+1, v);
    // }


    //Fill in the blank to make the last printlin! work

    //A counter variable

    // let mut n: i32 = 1;

    // //Loop while the condition is true

    // while n  < 10
    //   {
    //     if n % 15 == 0{
    //         println!("fizzbuzz");
    //     }else if n % 3 == 0 {
    //         println!("fizz");
    //     }else if n % 5 == 0{
    //         println!("buzz")
    //     }else{
    //         println!("{}",n);
    //     }

    //     n +=1;
    // };

    // let mut n = 0;

    // for i in 0..=100{
    //     if n == 66{
    //         print!("{}",n);
    //         break;
    //     }
    //     n+=1;
    // }

    // assert_eq!(n, 66);

    // println!("success");


    //fill in the blank

    // let mut  n  = 0;

    // for i in 0..=100{
    //     if n != 66{
    //         n+=1;
    //         println!("{} in n! == 66", n);
    //         continue;
    //     }

    //     println!("{}", n);
    //     break;
    // }

    // assert_eq!(n, 66);

    // println!("Success");

    // let mut count: u32 = 0u32;

    // println!("Let's count until infinity!");

    // //Infinite loop

    // loop{
    //     count += 1;

    //     if count == 3{
    //         println!("three");

    //         //sskip the rest of the code

    //         continue;
    //     }
    //     println!("{}", count);

    //     if count == 5{
    //         println!("ok, that's enount");

    //         break;
    //     }

    // }

    
    // assert_eq!(count, 5);

    // println!("success");


    // let mut counter: i32 = 0;

    // let result: i32 = loop {
    //     counter +=1;

    //     if counter == 10{
    //         // println!("{}", counter);

    //         break counter * 5;
    //     }
    // };

    // assert_eq!(result, 50);

    // println!("Success");


    let mut count = 0;

    'outer: loop {
       'inner1: loop {
        if count >= 20{
            //This would break only the inner1 loop
            break 
            'inner1;  //`break` is also works.
        }
        count +=2;
        // println!("count {}", count)
       }


       count += 5;

       'inner2: loop {
           if count >= 30{
            //This breaks the outer loop
            break 'outer;
           }

           //this will continue the outer loop
           continue 'outer;
       }

    }

    assert!(count == 30);

    println!("Success");

}