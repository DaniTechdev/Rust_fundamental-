//make it work in two ways, none of them is to remove `take(movable)` away from the

// fn main(){
//     let movable: Box<i32> = Box::new(3);

//     let consume =  || {
//         println!("`movable`: {:?}", movable);

//         take(&movable);
//     };


//     consume();
//     consume();
// }


// fn take<T>(_v:&T){}



//for comparison the following code has no error

fn main(){
    let movable = Box::new(3);

    
    let consume =move  || {
        println!("`movable`: {:?}", movable);

       
    };


    consume();
    consume();
}