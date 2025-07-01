
#[derive(Debug)]
// struct Color(i32,i32,i32);
// struct Point(i32,i32,i32);

struct Person{
    name:String,
    age:u8,
}

struct  User {
    active:bool,
    username: String,
    email:String,
    sign_in_count:u64,
}
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}


fn main(){
    //Tuple Structs

    //  struct  Color(i32,i32,i32);
    //  struct Point(i32,i32,i32);

    //  let black = Color(0,0,0);
    //  let origin = Point(0,0,0);

     //Unit-Like Stucts

     //exercices


    //  //fix the error
    //  struct Person{
    //     name:String,
    //     age:u8,
    //     hobby:String
    //  }


    //  let age: u8 = 30;
    //  let p: Person = Person{
    //     name:String::from("sunface"),
    //     age,
    //     hobby:String::from("coding"),
       
    //  };

    //  println!("success")


    // struct Unit;

    // trait SomeTrait {
    //     // ...Some behaviour defined here.
    // }


    // //We don't care about what fields are in the unit, but we caare about its behavour
    // //So we use a struct wih no field and implement some behaviours for it

    // impl  SomeTrait for Unit {
    //     let u = Unit;
    //     do_something_with_unit(u);

    //     println!("success");
    // }



    //fix the error and fill the blanks
    // struct Color(i32,i32,i32);
    // struct Point(i32,i32,i32);

    // let v: Point = Point(0,127,255);
    // check_color(v);

    // println!("success");


    //Fill the blank and fiex the error wihout adding/removing new line

 

    // let age: u8 = 18;
    // let mut p: Person = Person{
    //     name:String::from("Sunface"),
    //     age
    // };

    // //How can you believe sunface is only 18?
    // p.age = 30;

    // //fill the blank

    // p.name = String::from("sunfei");

    // println!("Success");


  
  
  //Fill the blank to make the code to work


//   let u1: User = User{
//     email:String::from("natochi@gmail.com"),
//     username:String::from("sunface"),
//     sign_in_count:1,
//     active:true
//   };

//   let u2: User = set_email(u1);

//   println!("success");


//fill the blanks to make the code work



// let scale: u32 = 2;
// let rect1: Rectangle = Rectangle{
//  width:dbg!(30 * scale), //Print debug info to stderr and assign the value of `30 * scale ` to `width`
//  height:50,
// };


// dbg!(&rect1); //print debug info to stderr

// println!("{:?}",rect1); //Print debug info to stdout



//Partial move


#[derive(Debug)]
struct  Person{
    name:String,
    age:Box<u8>,
}

let person: Person = Person{
    name:String::from("Natochi"),
    age:Box::new(20),
};

//`name` is moved out of person , but `age ` is referenced

let Person {name, ref age} = person;

println!("The person's age is {}", age);
println!("The person's name is {}", name);

//Error! borrow of partially moved value: `person` partial move occurs
//printLn!("The person struct is {:?}", person);

//`person` cannot be used but `person.age` can be used as it is not moved

println!("The person't age from person struct is {}", person.age);


}

// fn set_email(u: User) -> User{
//     User{
//         email:String::from("tochidan11@gmail.com"),
//         ..u
//     }
// }
// fn build_person( name:String,age:u8)-> Person{
//     Person { name, age }
// }

// fn check_color(p:Point){
    
//     let Point(x, _,z) = p; //we don't want the value of y;
//     assert_eq!(x,0);
//     assert_eq!(p.1,127);
//     assert_eq!(z,255);

//     // println!(" x , y z are {},{},{}", x, y,z);
// }

// fn do_something_with_unit(u:__){}