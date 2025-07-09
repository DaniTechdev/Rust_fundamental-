//Associated Types

trait MyTrait {
    type MyType;

    fn get_my_type(&self) -> Self::MyType;
}


struct MyStruct{}

impl MyTrait for MyStruct {
    type MyType = i32;

    fn get_my_type(&self) -> Self::MyType {
        return 42;
    }
}