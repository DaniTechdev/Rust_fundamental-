// struct Owner(i32);

// impl Owner{
//     //Annotatenlifetime as in a standalone function

//     fn add_one<'a>(&'a mut self) {self.0 +=1}
//     fn print<'a>(&'a self){
//         println!("`print`: {}", self.0);
//     }

// }

// fn main(){
//     let  mut owner = Owner(18);

//     owner.add_one();
//     owner.print();
// }

//Make it work by adding proper lifetime annotation

// struct ImportantExcert<'a>{
//     part:&'a str,
// }
struct ImportantExcert{
    part:&'static str,
}

// impl ImportantExcert<'_>{
//     fn level<'a>(&'a self)-> i32{
//         3
//     }
// }
impl ImportantExcert{
    fn level(& self)-> i32{
        3
    }
}

fn main(){}