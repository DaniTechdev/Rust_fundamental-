//Fix the error

fn main(){
    let mut vec = Vec::with_capacity(10);

    //The vector contains no items, even thoough it has capacity for more
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    //These are all done without reallocating memory

    for i in 0..10{
        vec.push(i);
    }

    // println!("{:?}", vec);

    assert_eq!(vec.len(),10);
    assert_eq!(vec.capacity(),10);

    println!("Capacity after pushing 10 elements: {}", vec.capacity());

    //..but this may make the vector reallocate memory
    vec.push(11);
    assert_eq!(vec.len(),11);
    assert!(vec.capacity() > 10); //the capacity may have changed
    println!("Capacity after pushing 11th element: {}", vec.capacity());


    //Fill in an appropraite value to amke the `for` done without reallocating memory

    let mut vec = Vec::with_capacity(100);
    for i in  0..100{
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success");
    //Note: the capacity may change if you push more than 100 elements

}