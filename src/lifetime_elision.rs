//Remove all the lifetimes thaat can be elided
#[warn(unused_variables)]
fn input(x: & i32){
    println!("`annotaated_input`: {}", x);
}
// fn input<'a> (x: &'a i32){
//     println!("`annotaated_input`: {}", x);
// }

fn pass(x: &i32) -> &i32{x}
// fn pass<'a> (x:&'a i32) -> &'a i32{x}

fn longest <'a,'b> (x:&'a str, y:&'b str) -> &'a str{
    x
}

struct Owner(i32);

impl Owner{
    //Annotate lifetimes as in a standalone function
    fn add_one (& mut self) {self.0 +=1;}
    fn print (& self){
        println!("`print`: {}", self.0);
    }
}

// struct Person<'a> {
//     age:u8,
//     name:&'a str,
// }

struct Person {
    age:u8,
    name:&'static str,
}

enum Either<'a>{
  Num(i32),
  Ref(&'a i32)
}

fn main(){

}