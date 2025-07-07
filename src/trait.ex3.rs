
use std::ops;

//Implement fn multiply to make the code work.
//As mentioned above, `+` needs `T` to implement `std::ops::Add` Traits.
//so, what about `*` ? You can find the answer to the Rust documentation


fn multiply<T: std::ops::Mul<Output = T>>(a:T,b:T)->T{
    a * b //amul(b)
}

fn main(){
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0,5.0));

    print!("Success");
}