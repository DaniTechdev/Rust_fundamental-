// use utf8_slice;

// fn main(){
//     let s =  "The # goes to the moon!";

//     let  rocket = utf8_slice::slice(s,4,5);
//     //will equall "#"
// }


//Fill in the blanks

fn main(){
    let mut s:String = String::new(); //Vec<u8> -> String-> "hello"
    s.push_str("hello");

    //Some bytes, in a vector
    let v: Vec<u8> = vec![104, 101, 108, 108, 111, ]; //hello 104 = 'h', 101 = 'e', 108 = 'l', 108 = 'l', 111 = 'o'

    //Turn a byte's vector into a String
    let s1:String = String::from_utf8(v).unwrap();

    assert_eq!(s,s1);

    println!("success!");
}