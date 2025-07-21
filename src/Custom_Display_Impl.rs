//Make it work

use std::fmt;


struct List(Vec<i32>);

impl fmt::Display for List{
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {

        //Extract the value using tuple indexing,
        //and create a reference to `vec`.

        let vec: &Vec<i32> = &self.0;


        write!(f, "[") ;

        //Iterate over `v` in `vec` while enumerting the iteration
        //count in `count`

        for (count, v) in vec.iter().enumerate(){
            //For every element except the first, add a coma.
            //use the ? operation to return on errors

            if count != 0 { write!(f, ", ")?;}
            write!(f,"{}: {}", count,v)? ;


        } 

            write!(f, "]") 

    }
}

fn main(){
    let v:List = List(vec![1,2,3]);
    assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");

    println!("{}", v);
    println!("Success");
}