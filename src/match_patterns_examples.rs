struct Point {
    x:i32,
    y:i32
   }
   
   
   // enum Message{
   //     Hello{id:i32}
   // }
   fn main(){
       //Patterns
   
       //Use | to match several values, use ..= to match an inclusive range
       
       // match_number(3);
   
   
       // The @  operator lets use create a variable that hold a value, at the saame time
       //are testing that value to see whether it matches a pattern
       
   
       //Fil in the blank to let p match the second arm
   
       // let p: Point = Point{x:4,y:10};
   
   
       // match p {
       //     Point{x,y:0} => println!("On the x axis at {}",x), //note x inside a struct means x:x Point{x,y:0} = Point{x:x,y:0}
       //     //Second arm
       //     Point{ x:0..=5, y:y@ (10|20|30)} => println!("On the y axis at {}",y),
       //     Point{x,y}=> println!("On neither axis: ({},{})",x,y),
       // }
   
   
       //Fix the errors
   
   //  let msg: Message = Message::Hello { id: 5};
   
   // match msg {
   //     Message::Hello { id:id@3..7 }
   //     => println!("Found an id in  range [3,7]: {}",id),
   //     Message::Hello { id: newid@ (10| 11 |12 )}=> {
   //         println!("Found an id in another range [10,12]: {}", newid)
   //     },
   //     Message::Hello { id } => println!("Found some other id: {}",id),
   //   }
   
   //Fill in the blank to make `split` MUST be used
   
   // let num: Option<i32> = Some(4);
   // let num: Option<i32> = None;
   // let num: Option<i32> = Some(4);
   
   
   
   
   // let split: i32 = 5;
   
   // match num {
   //     Some(x) if x < split  => (assert!(x< split)),
   //     Some(x) => assert!(x >=split),
   //     None => println!("None matched"), 
   // }
   
   
   
   //Ignoring remaing parts of the value with ..
   //fill th blank to make parts of the code to work
   
   
   // let numbers  = (2,4,8,16,32,64,128,256,512,1024,2048);
   
   // match numbers {
   //     (first,..,last) => {
   //         assert_eq!(first,2);
   //         assert_eq!(last, 2048);
   //     }
   // }
   
   
   //Using pattern &mut v to match a mutable reference needs you to be very caarefullc, due to v
   //being a value after matching
   
   //Fix the error with least  changing
   //Don't remove any code line
   
   let mut v = String::from("Hello,");
   
   let r: &mut String =  &mut v;
   
   match r{
        value => value.push_str(" world"),
   }
   
   
   println!("{}", v);
   println!("Success");
   }
   
   
   
   // fn match_number(n:i32){
   //     match n {
   //         //match a single value
   //         1 => println!("One!"),
   
   //         //Fill in the blank with `|` , DON@T use `..` or `..=`
   
   //         2 | 3 | 4 | 5 => println!("mach 2 -> 5"),
   //         //Match an inclusive range
   
   //         6..=10 =>{
   //             println!("match 6-> 10")
   //         },
   
   //         _ => println!("match - infinite -> or 11 -> +infinite")
   //     }
   // }