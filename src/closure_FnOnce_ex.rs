// fn main(){
//     let s =  String::new();

//     let updated_string = move || println!("{}",s);

//     exec(updated_string);
// }

// fn exec<F:FnOnce()> (f:F){
//     f()
// }

// fn main(){
//     let s =  String::new();

//     let updated_string = move || println!("{}",s);

//     exec(updated_string);
// }

// fn exec<F:Fn()> (f:F){
//     f()
// }



fn main(){
    let mut s =  String::new();

    let updated_string = |str|-> String {s.push_str(str); s}; //closure use the capture value by value because we are returning s and to return something you must be the owner

    exec(updated_string);
}

fn exec<'a, F:FnOnce(&'a str) -> String> (mut f:F){
    f("hello");
}