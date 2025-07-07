//fix the errors, DON'T modify the code in `main`.
use std::ops; //ops module from standard library

struct Foo;
struct Bar;

#[derive(Debug, PartialEq)]
struct FooBar;

#[derive(Debug, PartialEq)]

struct BarFoo;

//The `std::ops::Add` trait is used to specify the functionality of `+`
//Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
//The following block implements the operation: Foo + BAr =FooBar


//Foo + Bar -> Foo.add(Bar) //The rhs is Bar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs:Bar) -> FooBar{
        FooBar
    }
}


//Foo-Bar -> Foo.sub(Bar)

impl ops::Sub<Bar> for Foo{
    type Output = BarFoo;

    fn sub(self,_rhs:Bar)-> BarFoo{
        BarFoo
    }
}

fn main(){
    //Don't modify the code below
    //You need to derive some trait for Foobar to make it comparable

    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar , BarFoo);

    println!("Success");
}
