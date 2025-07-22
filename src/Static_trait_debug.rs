//make it work

use std::fmt::Debug;

fn print_it<T:Debug + 'static> (input:T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1 (input: impl Debug + 'static){
    println!("'static value passed in is: {:?}", input);

}

fn print_it2<T: Debug + 'static> (input: &T){
    println!("'static value passed in is: {:?}", input);

}


fn main(){
    //i is owned and contains no reference , thus it's static
    // const i: i32 = 5;
    static  i: i32 = 5;
    print_it(i);

    //oops, &i only has the lifetime defined by the scope of 
    //main(), so it's not 'static;

    print_it1(&i);

    //but this one WORKS!

    print_it2(&i);
}


use std::fmt::Display;

// fn main(){
//     let mut string = "First".to_string();

//     string.push_str(string.to_uppercase().as_str());

//     print_a(&string);
//     print_b(&string);
//     print_c(&string);
//     print_d(&string);
//     print_e(&string);
//     print_f(&string);
//     print_g(&string);
// }


// fn print_a<T: Display + 'static> (t:&T){
//     println!("{}", t);
// }