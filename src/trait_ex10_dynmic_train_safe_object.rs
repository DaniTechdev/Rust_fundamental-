// //Use the least two approaches to make it work.
// //Don't add/remove any code line.

// trait MyTrait {
//     fn f(&self)-> Self;
// }

// impl MyTrait for u32{
//     fn f(&self)-> Self {
//         42
//     }
// }

// impl MyTrait for String{
//     fn f(&self)-> Self {
//         self.clone()
//     }
// }


// //firstly lets go with static dispatch
// fn my_function<T: MyTrait>(x:T) -> T{
//     x.f()
// }

// //the compiler will use static dispatch to call the method f
// //for the specific type of T at compile time.
// //e.g. if T is u32, it will call u32::f, and if T is String, it will call String::f.

// //eg for u32 will be like this:
// // fn my_function_u32(x:u32)->u32{
// //     x.f();
// // }

// // for string
// //fn my_function_string(x:String)->String{
// //     x.f();
// // }
// fn main(){
//     my_function(13_u32);
//     my_function(String::from("abc"));
// }


//Use the least two approaches to make it work.
//Don't add/remove any code line.


//Before we can use the dynamic dispatch on the trait Object, we need to make the trait object safe.
// This means that the trait must not have any methods that return `Self` or take `Self` as a parameter.
// This is because the size of `Self` is not known at compile time, and the compiler cannot generate a vtable for it.
// To make the trait object safe, we can change the method signature to return a `Box<dyn MyTrait>` instead of `Self`.
// This allows us to return a heap-allocated trait object that can be used with dynamic dispatch.


trait MyTrait {
    fn f(&self)-> Box<dyn MyTrait>;
}

impl MyTrait for u32{
    fn f(&self)->  Box<dyn MyTrait> {
        Box::new(42)
    }
}

impl MyTrait for String{
    fn f(&self)->  Box<dyn MyTrait> {
       Box::new( self.clone())
    }
}




//using dynamic dispatch
fn my_function(x:Box<dyn MyTrait>) -> Box<dyn MyTrait> {
    x.f()
}

fn main(){
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));
}
