//Variables, Functions, and Basic Operations in Rust

//In rust a variable is immutable by default, to make it mutable we need to use the mut keyword.
fn main() {
    // let x:i32= 132; //initailized but used, error!
    // let _y:i32 = 132; //unintialized but also unused only  warning!
  
    // assert_eq!(x,132);
    // println!("Successfully executed the code!");
    
  
    // {
    //   let animal=    "dog";
    //   println!("The animal is a {}", animal);
    // }
  
    // fn checkname(){
    //   let name: &str = "Alice";
    //   println!("hello, welcome {}", name);
    // }
  


    let s =String::from("Hello, Rust!"); // String is a heap-allocated, growable string type in Rust, it comes into scope here


    takes_ownership(s); //s's value is moved into the function, 
                        //and s is no longer valid here

    let x = 5;  // x is an integer, which is a Copy type in Rust, so it can be copied and it comes into scope here

    makes_copy(x); // x is moved into the function, 
                    //but since x is i32  a Copy type, it is still valid here

                    //x is used after words

  }

  fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("Taking ownership of: {}", some_string);
    // some_string is dropped here when it goes out of scope
  } //memory is freed


  fn  makes_copy(some_inteer:i32){ //some_integer comes into scope
    println!("{}", some_inteer);
  }// Here, some_integer goes out of scope. Nothing spacial happe
  
  
  
  
  // fn greeting(){
  // let name: &str = "Alice";
  // println!("hello, welcome {}",name)
  // }
  
  
  // fn add(){
  //     let a: i32 = 5;
  //     let b:i32 = 10;
  //     let sum:i32 = a+b;
  //     println!("The sum of {} and {} is {}", a,b, sum)
  // }

  fn main(){

    let s1 = gives_ownership(); //gives_ownership moves its return 
                                // value into s1        

    let s2 =String::from("hello"); //s2 comes into scope

    let s3 = takes_and_gives_back(s2); //S2 is moved into
                                        //takes_and_gives_back, which also
                                        //takes ownership of s2 and move its return value into s3.

}

    //Here, s3 goes out of scope and is dropped. s2 moved , so nothing
    //happen. s1 goes out of scope and is dropped.


fn gives_ownership()-> String{ //gives_ownership  will move its return 
                                //vaalue into the function that calls it.

    let some_string =  String::from("yours"); //some_string comes into scope

                                
                               
    some_string                     //some_string is returned and
                                //move out to the calling function

}

//This function takes a string and returns one
fn takes_and_gives_back(a_string:String)-> String{ //a_string comes into scope
 
 a_string //a_string is returned and moved out to the calling function
}
  
  
  