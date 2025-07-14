#[derive(Debug)]

struct Number{
    value:i32
}


impl From<i32> for Number{
    fn from(n:i32) -> Number{
        Number { value: n }
    }
}

fn main(){
    let num = Number::from(30);
    assert_eq!(num.value,30);

    // let num:Number = 30_i32.into();

    println!("Success");
}


