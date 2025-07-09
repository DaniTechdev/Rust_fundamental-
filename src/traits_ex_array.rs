trait Bird{
    fn quack(&self);
}

struct Duck;

impl Duck{
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}

struct Swan;

impl Swan {
    fn fly(&self){
        println!("look, the duck.. oh sorry, the swan is flying");
    }
}

impl Bird for Duck{
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl  Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan")
    }
}


fn main(){
    //Fill in the blank to make the code work.

    // let birds:[Box<dyn Bird>;2] = [Box::new(Swan),Box::new(Duck)];


    // for bird in birds{
    //  //When duck and swan turn into birds, they all forgot how to fly, only remeneber
    //  //so the code below will cause an error

    //  //bird.fly();
    //   bird.quack()
    // }


    //note a &ref and a pointer are all references
    //the code above can be rewritten as

    let birds:[&dyn Bird;2] = [&Swan,&Duck]; // usize //the size of array must be knwon at compile time

    
    for bird in birds{
        //When duck and swan turn into birds, they all forgot how to fly, only remeneber
        //so the code below will cause an error
   
        //bird.fly();
         bird.quack()
       }


}