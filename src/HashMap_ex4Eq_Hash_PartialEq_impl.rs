//Fix the errors

//Tips: `derive` is usually  a good way to implements some common used traits

use std::collections::HashMap;
#[derive(Debug, Eq,  Hash, PartialEq)]

struct Viking{
    name:String,
    country:String
}


impl  Viking {
    fn new(name:&str, country:&str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string()
        }
    }
}


fn main(){
    //use a HashMap to store Vikings' health points

    let vikings: HashMap<Viking, i32> = HashMap::from([
        (Viking::new("Ragnar", "Denmark"), 100),
        (Viking::new("Lagertha", "Norway"), 90),
        (Viking::new("Bjorn", "Sweden"), 80),
    ]);


    //Use derived implementation to print the status of the vikings

    for (viking, health) in &vikings{
        println!("{:?} has {} hp", viking,health )
    }
}