

// struct Rectangle{
//     width:u32,
//     height:u32
// }

// impl Rectangle{
//     //Complete the area method which return the area of a rectangle
//     fn area(self) -> u32{
//         self.width * self.height
//     }
// }

//Note: self will take the ownership of current struct instance, however, &self will borrow
//a reference from the instance

#[derive(Debug)]
struct Trafficlight{
    color:String
}

// impl Trafficlight{
//     // pub fn show_state(self:&mut Self){
//     //     println!("the current state is {}", self.color);
//     // }

//     // //Fill in the blank, DON't use any variants pf `self`
//     // pub fn chnage_state(&mut self){
//     //     self.color = "green".to_string()
//     // }

//     //self:&mut Self) is same as &mut self
// }

impl Trafficlight{
 //1: Implement an associated function `new`
 //2: It will return a TrafficLight contains color `red`
 //3: Must use `self`, Don't use `Trafficlight` in fn signature or body
 //pub fn new()

 pub fn get_state(&self) -> &str{
    &self.color
 }


 pub fn new()->  Self{
   Self {
        color:String::from("red"),
    }
 }
}




fn main(){

    // let light: Trafficlight = Trafficlight::new();
    // assert_eq!(light.get_state(), "red");
    // println!("success");


    // let light = Trafficlight{
    //     color: "red".to_owned(),
    // }
    // //Don't take the ownership of `light` here.
    // light.show_state(),
    // //...otherwise there will be an error below
    // let rec1:Rectangle = Rectangle { width: 30, height: 50 };
    // assert_eq!(rec1.area(), 1500);
    // println!("success");
}