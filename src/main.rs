fn main (){
    //ownership, memory(stack and heap of the RAM in Rust)

    let x: i32 = 32;
    let y: i32= 10;

    let z = add_numbers(x,y);

    println!("the result is {}", z);
}

fn add_numbers (a:i32, b:i32)-> i32{
    let c = a + b;
    c
}


