
#[allow(overflowing_literals)]

// fn main(){
//     assert_eq!(u8::MAX, 255);
//     //The max of u8 is 255 as shown above.

//     //so the below code will cause an overflow error:literal out of range fo u8;

//     //please looking for clues within complie errors to FIX it.

//     //Don't modify any code in main

//     let v: u8 = 1000 as u8;

//     println!("success, the value of v is: {}", v);


// }
fn main(){
    assert_eq!(1000 as u16, 1000); // 1000 fits in u16, so it is converted directly
    assert_eq!(1000 as u8, 232); // 1000 exceeds u


    //For positive numbers, this is the same as the modulus

    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255);

    //Since Rust 1.45 , the `as` keyword performs a *saturating cast*
    //when casting from float in int. If the floating point value exceeds
    //the upper bound or is less than the lower bound, the returned value
    //will be equal to the bound crossed.

    assert_eq!(3000.1_f32 as u8, 255); // 3000.1 exceeds u8's upper bound, so it saturates to 255
    assert_eq!(-100.0_f32 as u8, 0); // -100.0 is less than u8's lower bound, so it saturates to 0


    //This behavior incurs a small runtime cost and can be avoided
    //with unsafe methods, however the results might overflow and
    //return ** unsound values **. Use these methods wisely

    unsafe{
        //300.0 is 44 -> 300 - u8::MAX (255) +1 = 300-256 = 44
        println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());
        //-100.0 as u8 is 156 -> -100 + u8::MAX (255) +1 = 156
        println!("-100.0 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());
        //nan as u8 is 0
        println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }
}