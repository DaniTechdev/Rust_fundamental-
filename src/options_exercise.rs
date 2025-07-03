// enum Option<T>{
//     None,
//     Some(T),
// }

fn main(){
    //fill in the blank to make the `println` work
    //Also add some code to prevent the `panic` from running

    let five:Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five); //Some(6)
    let none: Option<i32> = plus_one(None);

    if let Some(n) =six {
        println!("{}",n);

        println!("success");
    }else {
        panic!("Never let this run!");
        
    }

}

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i)=> Some(i+1),
    }
}