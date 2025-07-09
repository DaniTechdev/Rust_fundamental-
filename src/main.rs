trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String{ format!("u8:{}", *self) }
    
}

impl Foo for String {
    fn method(&self) -> String { format!("String:{}", self) }
}


//Implement below with generics

fn static_dispatch<T: Foo>(a:T){
    a.method();
}


fn dynamic_dispatch(a:&dyn Foo){
    a.method();
}


fn main(){
    let x: u8 = 5u8;
    let y: String = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);
}
