

fn main(){
    assert_eq!(sum(1, 2), 3);

    println!("{}", sum(3.4, 5.4));
}


//Implement `fn sum` with bound in two ways;

fn sum<T:std::ops::Add<Output = T>> (a:T,b:T)->T{
    a+b //a.add(b) //or a + b
}