//Variables, Functions, and Basic Operations in Rust

//In rust a variable is immutable by default, to make it mutable we need to use the mut keyword.
fn main() {
    let x:i32= 132; //initailized but used, error!
    let _y:i32 = 132; //unintialized but also unused only  warning!
  
    assert_eq!(x,132);
    println!("Successfully executed the code!");
    
  
    {
      let animal=    "dog";
      println!("The animal is a {}", animal);
    }
  
    fn checkname(){
      let name: &str = "Alice";
      println!("hello, welcome {}", name);
    }
  
  }
  
  
  
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
  
  
  