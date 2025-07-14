#[allow(overflowing_literals)]

fn main(){
    assert_eq!(u8::MAX, 255);
    //The max of u8 is 255 as shown above.

    //so the below code will cause an overflow error:literal out of range fo u8;

    //please looking for clues within complie errors to FIX it.

    //Don't modify any code in main

    let v: u8 = 1000 as u8;

    println!("success, the value of v is: {}", v);


}