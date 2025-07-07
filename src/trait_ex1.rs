//traits

//exercise1

//Don't modify the code in `main`.

trait Hello {
    //Default implementation of the method
    fn say_hi(&self) -> String{
        String::from("Hi")
    } 

    fn say_something(&self)-> String;
}



struct Student {}

impl Hello for Student  {

    fn say_hi(&self)->String{
        String::from("hi")
    }

    fn say_something(&self)-> String {
        String::from("I aam a good student")
    }
}

struct Teacher {}

impl  Hello for Teacher {
    fn say_hi(&self)->String{
        String::from("Hi,I am your new teacher")
    }

    fn say_something(&self)-> String {
        String::from("I am not a bad teacher")
    }
}


fn main(){

let s:Student = Student {};
assert_eq!(s.say_hi(),"hi");
assert_eq!(s.say_something(),"I aam a good student");

let t: Teacher = Teacher {};

assert_eq!(t.say_hi(),"Hi,I am your new teacher" );
assert_eq!(t.say_something(),"I am not a bad teacher");

println!("Success")
    
}