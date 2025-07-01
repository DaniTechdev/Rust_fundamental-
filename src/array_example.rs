
fn main(){
    //Slice is a reference to a collection. It just views the element without taking ownership 

    // let a = [1,2,3,4,5];

    // let slice: &[i32] = &a[1..3];

    // assert_eq!(slice, &[2,3]);

    //slice above has the type &[i32] in this example

    //works like string slices do, ny storing a refernce to the first 
    //element and a length.



    //Slices aare similar to arrays , but their length is not known at compile time, so
    //you can't use slice directly.



    //Here, both [i32] and str are slices types, but directly using it will cause errors.
    //You have to use the reference of the slice instead: &[i32], &str.

    //fix the errors, don't aadd new lines!
    

    // let arr: [i32; 3] = [1,2,3];

    // let sl:&[i32] = &arr[0..2]; //&[1,2]

    // let s2:&str = "Hello, world";

    // println!("success");


    // let arr:[char;3] = ['⛳','⛳','⛳'];

    // let slice: &[char] =  &arr[..2];//⛳⛳

    // //Modify '8' to make it work
    // //TIPS: slice(reference) is NOT an arraay, if it is is an aarray, then `aassert` will be passed: Each of the chars ⛳ aand ⛳ occuoy 4 bytes 2*4 = 8;


    // assert!(std::mem::size_of_val(&slice)== 16);

    // let arr: [i32;5] = [1,2,3,4,5];

    // //Fill the blanks to make the code work
    // let slice:&[i32] = &arr[1..4];

    // assert_eq!(slice,&[2,3,4]);

    // println!("success");
    


    // let s: String = String::from("Hello");

    // let slice1: &str = &s[0..2]; //&['h','e']

    // //Fill th blank to maake the code work, DON'T USE 0..2 again.
    // let slice2: &str = &s[..2];

    // assert_eq!(slice1,slice2);

    // println!("success");

    // let s:&str = "⛳⛳, ⛳⛳";

    // //modify this line to make the code work

    // let slice: &str = &s[0..3]; //note: slice of string accesses by bytes not index 

    // assert!(slice =="⛳");

    // println!("success");


  //&String can be converted to &str

  //fix errors

  let mut s: String = String::from("Hello, world");

  //Here ,&s is `&String` type, but `first_word` need a `&str` type.
  //It works because `&String` can be implicitly converted to `&str`. If you wan 

  let word: &str =  first_word(&s); //&String -> &str explicitly by Rust compiler
  println!("the first word is: {}", word);

  s.clear(); //error

//   println!("the first word is: {}", word);




}

fn first_word(s:&str)-> &str{
    &s[..1]
}