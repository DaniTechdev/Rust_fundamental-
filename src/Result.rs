//Fill in the blank to display the whole call stack
//Tip:you can find the clue in the default panic info


fn divide(x:f32, y:f32) -> Result<f32,&'static str>{
    if y == 0.0 {
        return Err("Division by zero");
    }
    Ok(x/y)
}

fn main(){
    let result: Result<f32, &'static str> = divide(10.0, 2.0);

    match result {
        Ok(val) => println!("Result: {}", val),
        Err(msg)=> println!("Error: {}",msg)
    }
}
