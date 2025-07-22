//make it work by changing the trait bount in two ways

fn fn_once<F> (func:F)
where 
 F:Fn(usize) -> bool, //defining trait bound using fn here instead of fnOnce
 {
    println!("{}", func(3));
    println!("{}", func(4));
 }


 fn main(){
    let x: Vec<i32> = vec![1,2,3];

    fn_once(|z| {z==x.len()});
 }