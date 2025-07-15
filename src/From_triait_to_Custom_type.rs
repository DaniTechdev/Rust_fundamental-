//From is now in `std::prelude`, so there is no need to introduce it into
//use std::convert::From

#[derive(Debug)]

struct Number{
    value:i32,
}

impl From<i32> for Number {
    //Implement `from` method
    //  fn from(value: T) -> Self; from the Rust documentation
    fn from(n:i32) ->Self{
        Self { value: n }
    }
}

//Fill in the blanks

fn main(){
    let num: Number = Number::from(30);

    assert_eq!(num.value,30);

    let num:Number = 30.into();
}