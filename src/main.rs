//Variables, Functions, and Basic Operations in Rust

fn main() {
    println!("Hello, world!");
    greeting();
    add();
}


fn greeting(){
let name: &str = "Alice";
println!("hello, welcome {}",name)
}


fn add(){
    let a: i32 = 5;
    let b:i32 = 10;
    let sum:i32 = a+b;
    println!("The sum of and {} is {}", a,b, sum)
}
