fn main(){


    //example 1
    // let mut s: String = String::from("Hello");


    //   change(&mut s);

    //   println!("{}",s);



//     let mut t: String = String::from("hello");

//   let r1: &mut String = &mut t; //This would violate the firts rule of
//                                 //borrowing, which says that we caan only 
//                                 //haave ONE mutable reference to the same data at a time
//   let r2: &mut String = &mut t;

//   println!("{}, {}", r1,r2);   



//example 2
// let mut s =  String::from("hello");

// {
//     let r1 = &mut s;
// }// r1 goes out of scope here, so we can make a new reference with no problems


// let r2 = &mut s;


//example 3

// let mut s = String::from("hello");

// let r1 = &s;//no problem
// let r2 = &s //no problem
// let r3  = &mut s; //Big problem


// //This would violate the first rule of borrowing, which says we either have any number of immutable
// //references or one single mutable references
// //l
// println!("{},{} and {}", r1,r2,r3);



// let mut s = String::from("hello");

// let r1 = &s;//no problem
// let r2 = &s ;//no problem

// println!("{},{} ", r1,r2);

// //variables r1 and r2 will not be used after this point

// let r3  = &mut s; //No problem

// println!("{}",r3);




}

// fn change(some_string:&mut String){
//     some_string.push_str(",World");
// }