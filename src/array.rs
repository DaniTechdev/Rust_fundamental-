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
    

    //All elements in an array caan be initialized to the saame value at once

    //Fill the blank

    // let list:[i32;100] = [1;100];//[1,1,1,1,...,1]

    // assert!(list[0]==1);
    // assert!(list.len() == 100);

    // println!("success");

    // //All elements in an array must be of the same type
    // let _arr: [i32; 3] = [1,2,3];


    // println!("success");


    //Indexing starts at 0
    // let arr:[char;3] = ['a','b','c'];
    
    // let ele = arr[0];

    // assert!(ele == 'a');

    // println!("success");




        //Out of bound indexing causes paanic
//fix the error
    // let names: [String; 2] = [String::from("Sunfri"), "Sunface".to_string()];

    // // `Get` returns an Option<T> it's safe to use but arr[index] will return the exact time accessed in the array


    // let name0: &String = names.get(0).unwrap();

    // //But indexing if not safe
    // let _name1: &String = &names[1];

    // println!("Success");



}

