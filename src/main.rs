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
    let s= String::from("Hellow,");

    //Modify this line only
    let mut s1: String = s;

    s1.push_str("world");

    println!("success");

}


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