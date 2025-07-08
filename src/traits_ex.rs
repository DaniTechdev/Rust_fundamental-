
struct  Sheep {}
struct Cow{}


trait Animal {
    fn noice (&self)-> String;
}

impl Animal for Sheep{
    fn noice (&self)-> String {
        "baaaaaah".to_string()
    }
}

impl Animal for Cow{
    fn noice (&self)-> String {
        "moooooo!".to_string()
    }
}


//Returns some struct that implements Animal trait, but we dont't know which one at compile time
//Fix the errors here, you can make a fake random, or you can use trait object.

// fn random_animal (random_number:f64) -> impl Animal{
    fn random_animal (random_number:f64) -> Box<dyn Animal> {

    if random_number < 0.5 {
        Sheep{}
    }else{
        Cow{}
    }
}


fn main(){
    let random_number: f64 = 0.234;
    let animal:Box<dyn Animal>=  random_animal(random_number);

    println!("You've randomly chosen an animal, it says {}", animal.noice());
}