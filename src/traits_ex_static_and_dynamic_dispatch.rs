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



//the above is a generic function, which allows for static dispatch at compile time.
//the compiler will generate a separate version of the function for each type that implements the `Foo` trait.
//e.g fn static_dispatch_u8(a:u8) { a.method(); } when the type is `u8`
// and fn static_dispatch_string(a:String) { a.method(); } when the type is `String`.
// This means that the method to be called is determined at compile time, based on the type of the object that `a` is.
// This is done by the compiler generating a separate function for each type that implements the `Foo` trait.
// The advantage of static dispatch is that it allows the compiler to optimize the code for each specific type,
// which can lead to better performance compared to dynamic dispatch.
//e.g. if you call `static_dispatch(5u8)` and `static_dispatch("Hello".to_string())`, the compiler will generate two separate functions, one for `u8` and one for `String`.
// This is known as monomorphization, where the compiler generates specific code for each type at compile time.
// This allows for better performance compared to dynamic dispatch, as there is no runtime overhead of looking up the method in a vtable.
// However, it also means that the code size can increase significantly if there are many different types that implement the `Foo` trait.
// This is a trade-off between performance and code size, and you should choose the one that fits your use case better.
// Note that this is not always the best choice, as it can lead to code bloat if there are many different types that implement the `Foo` trait.
// If you have a small number of types that implement the `Foo` trait, static dispatch is usually the better choice.
// If you have a large number of types that implement the `Foo` trait, dynamic dispatch is usually the better choice.
// If you have a mix of types that implement the `Foo` trait, you can use both static and dynamic dispatch, depending on the use case.





fn dynamic_dispatch(a:&dyn Foo){
    a.method();
}
//the above is a reference to a trait object, which allows for dynamic dispatch at runtime.
// This means that the method to be called is determined at runtime, based on the actual type of the object that `a` points to.
// This is done using a vtable, which is a table of function pointers that the compiler generates for each type that implements the `Foo` trait.
// The vtable is used to look up the method to be called at runtime, which adds some overhead compared to static dispatch.
// Dynamic dispatch is useful when you have a large number of types that implement the `Foo` trait, and you want to avoid code bloat.
// It allows you to write more generic code that can work with any type that implements the `Foo` trait, without having to know the specific type at compile time.
// However, it also means that there is a runtime overhead of looking up the method in the vtable, which can be significant if the method is called frequently.
// Dynamic dispatch is usually the better choice when you have a large number of types that implement the `Foo` trait, and you want to avoid code bloat.
// If you have a small number of types that implement the `Foo` trait, static dispatch is usually the better choice.
// If you have a mix of types that implement the `Foo` trait, you can use both static and dynamic dispatch, depending on the use case.
// Note that you can also use trait objects with generics, which allows you to have the best of both worlds.
// This is done by using the `Box<dyn Foo>` type, which is a heap-allocated trait object that can be used with generics.
// This allows you to have the performance of static dispatch with the flexibility of dynamic dispatch.
// However, it also means that there is a runtime overhead of allocating the trait object on the heap, which can be significant if the method is called frequently.
// This is a trade-off between performance and flexibility, and you should choose the one that fits your use case better.
//


fn main(){
    let x: u8 = 5u8;
    let y: String = "Hello".to_string(); //&String is a reference to a String, which is a heap-allocated string type in Rust.

    static_dispatch(x);
    dynamic_dispatch(&y);
}
