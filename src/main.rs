
// trait Animal {}
// #[derive(Debug)]
// struct Dog;
// #[derive(Debug)]
// struct Cat;


// impl Animal for Dog{}
// impl Animal for Cat{}

// fn return_animal(s:&str)-> &dyn Animal {
    
//     match s {
//         "dog"=> &Dog{},
//         "cat"=> &Cat{},
//         _ => panic!(),
//     }
// }



// fn main(){
//     let animal1 = return_animal("cat");
//     let animal2 = return_animal("dog");

//     println!("Animal 1: {:?}", animal1);        
// }



