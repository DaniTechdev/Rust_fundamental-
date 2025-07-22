//Fil in the blank

//A function which takes a closure aas an argument and calls it.
//<F> denotes that F is a "Generic type parameter"

fn apply<F>(f:F) where 
//The closure takes no input and returns nothing
F:FnOnce() {
    f();
}


//A function which takes a closure aand returns an `i32`

fn apply_to_3<F> (f:F) -> i32 where 
//The closure takes an `i32` and returns an `i32`.
    F:Fn(i32) -> i32
    {
        f(3) //|3| 2 * 3 = 6
    }


    fn main(){
        use std::mem;

        let greeting: &str = "hello";

        //A non-copy type.
        //`to_owned` creates owned data from borrowed one

        let mut farewell: String = "goodbye".to_owned();

        //Captured 2 variables `greeting` by reference and
        //`farewell` by value

        let diary = || {
            //`greeting` is by reference: requires `Fn`.

            println!("I said {}.", greeting);

            //Mutation forces `farewell` to be captured by
            //mutable reference. Now requires `FnMut`.
            farewell.push_str("!!!");

            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep.zzzzzz");

            //Manually calling drop forces `farewell` to 
            //be captured by value. Now requires `FnOnce`.
            mem::drop(farewell); //droping the value of farewell from memory
        };

        //call the function which applies the closure.

        apply(diary);

        //`double` satisfies `apply_to_3`'s trait bound

        let double = |x| 2 * x;

        println!("3 doubled:{}", apply_to_3(double));
    }