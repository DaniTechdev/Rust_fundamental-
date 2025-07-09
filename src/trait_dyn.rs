trait  Bird {
    fn quack(&self) -> String;
}

struct  Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}

struct Swan;
impl Swan{
    fn fly(&self){
        println!("Look, the duck.. oh sorry, the swam is flying")
    }
}

impl Bird for Duck{
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan{
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn main(){
    //Fill in the blank

    let duck: Duck = Duck;
    duck.swim();

    let bird: Box<dyn Bird>= hatch_a_bird(2);
    //This bird haas forgotten how to swim, so below line will cause an error.
    //bird.swim();
    //but it can quak,

    assert_eq!(bird.quack(), "duck duck");


    let bird:Box<dyn Bird> = hatch_a_bird(1);
    //This burd has forgotten how to fly, so below, line will cause an error.
    //bird.fly();
    //But it caan quack too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success");

    
}


//Implement this function

fn hatch_a_bird(x: u8) -> Box<dyn Bird>{

     match x {
         2 => Box::new(Duck),
         1 =>  Box::new(Swan),
         _ => panic!()
     } 

    //  if let x = 2{
    //    Box::new(Duck)
    //  }else let x = 1{
    //     Box::new(Swan)
    //  }else{ panic!()}
}