#[derive(Debug)]
struct  Rectangle{
    width:u32,
    height:u32
}


impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
}

impl Rectangle {
    fn new(width:u32, height:u32) ->Rectangle{
        Rectangle {
             width: width, height: height 
            }
        }
    }

fn main(){

    let rec1:Rectangle = Rectangle::new(5,10);

    println!("Rectangle: {:?}", rec1)
}



    // #[derive(Debug)]

    // struct Point{
    //     x:f64,
    //     y:f64,
    // }

    // //Implementation block, all `Point` aassociated functions & methods go in here.
    // impl Point{
    //     //This is an "associated function" becaause this function is associated with
    //     //a particular type, that is, Point.

    //     //Associated function don't need to be called wih an instance.
    //     //These functions are generally used like constructors.

    //     fn origin()-> Point{
    //         Point { x: 0.0, y: 0.0 }
    //     }

    //     //Aanother associated function, taking two arguments:
    //     fn new(x:f64, y:f64)->Point{
    //         Point { x: x, y: y }
    //     }
    // }