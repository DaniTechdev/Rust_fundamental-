use std::fmt;

struct Point{
    x:i32,
    y:i32
}

impl fmt::Display for Point{
    //Implement fmt method
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({},{})", self.x, self.y)
    }
}

fn main(){
    let orgin = Point{x:0,y:0};

    //Fill in the blanks
    assert_eq!(orgin.to_string(),"The point is (0,0)");
    assert_eq!(format!("{}",orgin),"The point is (0,0)");

    println!("{}",orgin);
    println!("Success");
}