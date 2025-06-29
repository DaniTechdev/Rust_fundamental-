fn main(){
    //Array

        //for example , you can't initilize aan aarray like below
    // fn init_arr(n:i32){
    //     let arr = [1;n];
    // }

    //This will cause an error because the compiler has no idea
    //of the exact size of the array at compile time.

     

    //  //fill the blank with proper arraay type
    // let arr:[i32;5] = [1,2,3,4,5];



    // //modify the code below to make it work

    // assert!(arr.len() == 5);
    // println!("success");


    //We can ignore parts of the aarray types or even the whole type aand let the compiler infer it for us
    // let arr0: [i32; 3] = [1,2,3];
    // let arr:[char;3]= ['a','b','c'];

    // //fill theblank
    // //Array are stack allocated, `std::mem::size_of_val` returns the bytes which
    // //A char takes 4 bytes in Rust: Unicode char

    // assert!(std::mem::size_of_val(&arr) == 12);

    // println!("success");
    

}

