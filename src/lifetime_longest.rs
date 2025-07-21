//Make it work  by aadding proper lifetime annotation

fn longest<'a>(x:&'a str, y:&'a str)-> &'a str {
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

fn main(){

    let x = "long";
    let y = "longer";

    println!("{}", longest(x, y));
}