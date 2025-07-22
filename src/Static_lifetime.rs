//Fil in the blank in two ways

// fn main(){
// let v:&'static str = "hello";
//     need_static(v);
// }

// fn need_static(r:&'static str){
//     assert_eq!(r,"hello")
// }


fn main(){

    let static_string: &'static str = "I'm in read-only memory";

    //make a `string` literal and print it:
    {

        println!("static_string: {}", static_string);
    }
        //when ` static_string` goes out of scope, the reference
    //can no longer be used but the data remains in the binary

    println!("static_string reference remaint alive: {}", static_string);
}