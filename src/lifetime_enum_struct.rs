//Make it work by adding proper lifetime annotation

//A type `Borrowed` which houses a reference to an 
//`i32` , The reference to `i32` must outlive `Borrowed`.

use core::num;

#[derive(Debug)]

struct Borrowed<'a>(&'a i32);

//Similarly, both references here must outlive this structure.

#[derive(Debug)]
struct NamedBorrowed<'a>{
    x:&'a i32,
    y:&'a i32
}

//An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a>{
    Num(i32),
    Ref(&'a i32)
}


fn main(){
    let x = 18;
    let y = 15;

    let single: Borrowed = Borrowed(&x);
    let double: NamedBorrowed = NamedBorrowed{x:&x,y:&y};
    let reference: Either = Either::Ref(&x);
    let number: Either = Either::Num(y);


    println!("x is borrowed in {:?}", reference);
    println!("x and y are borrowed in {:?}", double);
    println!("x is allowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}