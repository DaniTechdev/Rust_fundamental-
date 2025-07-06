use core::assert_eq;
use std::ops::Add;

// Implement the generic function below
// fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }


// struct Point<T>{
//     x:T,
//     y:T
// }


// struct Point<T,U>{
//     x:T,
//     y:U
// }

//Add generic to val to make the code work, DON't modify the code in `main`
// struct  Val<T>{
//     val:T
// }


// impl<T>  Val<T> {
//      fn value(&self) -> &T{
//         &self.val
//      }
// }



struct Point<T,U>{
    x:T,
    y:U
}


impl <T,U> Point<T,U>{
    //implement mexup to make it work, DON'T modify other code

    fn mixup<V,W> (self, other:Point<V,W>)-> Point<T,W>{
        Point { x: self.x, y: other.y }
    }
}



fn main() {

    let p1: Point<i32, i32> = Point{x:5,y:10};
    let p2: Point<&str, char> = Point{x:"Hello", y:'#'};

    let p3: Point<i32, char> = p1.mixup(p2);

    assert_eq!(p3.x,5);
    assert_eq!(p3.y, '#');

    println!("Success")
    // let x: Val<f64> = Val{val:3.0};
    // let y: Val<String> = Val{val:"Hello".to_string()};

    // println!("{}, {}", x.value(), y.value());

    // //Don't modify this code
    // let p: Point<i32, String> = Point{x:5, y:"hello".to_string()};


    // println!("Success");

    // assert_eq!(5, sum(2i8, 3i8));
    // assert_eq!(50, sum(20, 30)); // i32
    // assert_eq!(2.46, sum(1.23, 1.23)); // f64

    // println!("Success!");


    // //Implement struct Point to make it work.

    // let integer: Point<i32> = Point{x:5,y:10};

    // let float: Point<f64> = Point{x:1.0,y:4.0};

    // println!("success");



    //Modify this struct to make the code work


    
}