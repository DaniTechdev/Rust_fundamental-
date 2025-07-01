
fn main(){
    //Tuple

    
    // let _t0: (u8,i16) = (0,-1);

    // //Tuple can be tuple's member

    // let _t1: (u8, (i16,u32)) = (0,(-1,1));

    // //fil the blanks to make the code to work

    // let t:(u8,u16,i64,&str,String) = (1u8,2u16,3i64,"hello", String::from(",world"));

    // println!("Success");



    //Members can be extracted from tuple using indexing

    // let t:(&str,&str,&str) = ("i","am","sunface");

    // assert_eq!(t.1, "am");

    // println!("success");


    //Long tuple cannot be printed

    // let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);

    // println!("too long tuple: {:?}",too_long_tuple);


    //Destructuring tuple with pattern

    // let tup:(i32,f64, &str) = (1,6.4,"hello");

    // //fill the blank to make the code work

    // let (x,y,z): (i32, f64, &str) = tup;

    // assert_eq!(x,1);
    // assert_eq!(z,"hello");
    // assert_eq!(y,6.4);

    // println!("success");


    // let (x,y,z);

    // //Fill the blank

    // (y,z,x)= (1,2,3);

    // assert_eq!(x,3);
    // assert_eq!(y,1);
    // assert_eq!(z,2);

    // println!("success");


    let (x,y) = sum_multiply((2,3));

    assert_eq!(x,5);
    assert_eq!(y,6);

    println!("success");


}


fn sum_multiply(nums:(i32,i32))->(i32,i32){
    (nums.0 + nums.1, nums.0 * nums.1)
}